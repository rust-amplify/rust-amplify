// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::HashMap;
use std::iter::FromIterator;
use std::convert::TryInto;
use proc_macro2::{TokenStream as TokenStream2, Span, Ident};
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, Error, Fields, Result, LitStr, Attribute, DataStruct, ImplGenerics,
    TypeGenerics, WhereClause, Field,
};

use amplify_syn::{ParametrizedAttr, AttrReq, ArgReq, ArgValue, ValueClass, LiteralClass};

pub(crate) fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let struct_name = &input.ident;

    let mut global_param = ParametrizedAttr::with("getter", &input.attrs)?;
    let _ = GetterDerive::try_from(&mut global_param, true)?;

    match input.data {
        Data::Struct(data) => derive_struct_impl(
            data,
            struct_name,
            global_param,
            impl_generics,
            ty_generics,
            where_clause,
        ),
        Data::Enum(_) => Err(Error::new_spanned(
            &input,
            "Deriving getters is not supported in enums",
        )),
        Data::Union(_) => Err(Error::new_spanned(
            &input,
            "Deriving getters is not supported in unions",
        )),
    }
}

#[derive(Clone)]
struct GetterDerive {
    pub prefix: LitStr,
    // pub doc: Attribute,
    pub skip: bool,
    pub copy: bool,
    pub base: Option<LitStr>,
    pub main: LitStr,
    pub as_ref: LitStr,
    pub as_mut: LitStr,
}

impl GetterDerive {
    fn try_from(attr: &mut ParametrizedAttr, global: bool) -> Result<GetterDerive> {
        let mut map = HashMap::from_iter(vec![
            ("prefix", ArgReq::with_default("")),
            ("all", ArgReq::Prohibited),
            ("as_copy", ArgReq::with_default("")),
            ("as_clone", ArgReq::with_default("")),
            ("as_ref", ArgReq::with_default("_ref")),
            ("as_mut", ArgReq::with_default("_mut")),
        ]);

        if !global {
            map.insert("skip", ArgReq::Prohibited);
            map.insert(
                "base_name",
                ArgReq::required(ValueClass::Literal(LiteralClass::StringLiteral)),
            );
        }

        attr.check(AttrReq::with(map))?;

        if attr.args.contains_key("all") {
            if attr.args.contains_key("as_clone")
                || attr.args.contains_key("as_ref")
                || attr.args.contains_key("as_mut")
            {
                return Err(Error::new(
                    Span::call_site(),
                    "`all` attribute can't be combined with other",
                ));
            }
            attr.args.remove("all");
            attr.args.insert("as_clone".to_owned(), ArgValue::from(""));
            attr.args
                .insert("as_ref".to_owned(), ArgValue::from("_ref"));
            attr.args
                .insert("as_mut".to_owned(), ArgValue::from("_mut"));
        }

        if attr.args.contains_key("as_clone") && attr.args.contains_key("as_copy") {
            return Err(Error::new(
                Span::call_site(),
                "`as_clone` and `as_copy` attributes can't be present together",
            ));
        }

        Ok(GetterDerive {
            prefix: attr
                .args
                .get("prefix")
                .map(|a| a.clone().try_into())
                .transpose()?
                .expect("amplify_syn is broken"),
            skip: attr
                .args
                .get("skip")
                .cloned()
                .map(ArgValue::try_into)
                .transpose()?
                .unwrap_or_default(),
            copy: attr.args.contains_key("as_copy"),
            base: attr
                .args
                .get("base_name")
                .map(|a| a.clone().try_into())
                .transpose()?,
            main: attr
                .args
                .get("as_copy")
                .or(attr.args.get("as_clone"))
                .map(|a| a.clone().try_into())
                .transpose()?
                .expect("amplify_syn is broken"),
            as_ref: attr
                .args
                .get("as_ref")
                .map(|a| a.clone().try_into())
                .transpose()?
                .expect("amplify_syn is broken"),
            as_mut: attr
                .args
                .get("as_mut")
                .map(|a| a.clone().try_into())
                .transpose()?
                .expect("amplify_syn is broken"),
        })
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum GetterMethod {
    Main { copy: bool },
    AsRef,
    AsMut,
}

impl GetterMethod {
    fn all(copy: bool) -> [GetterMethod; 3] {
        [
            GetterMethod::Main { copy },
            GetterMethod::AsRef,
            GetterMethod::AsMut,
        ]
    }

    fn doc_phrase(&self) -> &'static str {
        match self {
            GetterMethod::Main { copy: true } => "returning copy of",
            GetterMethod::Main { copy: false } => "cloning",
            GetterMethod::AsRef => "borrowing",
            GetterMethod::AsMut => "mutating borrow",
        }
    }

    fn ret_prefix(&self) -> TokenStream2 {
        let s = match self {
            GetterMethod::Main { copy: true } => "",
            GetterMethod::Main { copy: false } => "",
            GetterMethod::AsRef => "&",
            GetterMethod::AsMut => "&mut",
        };
        quote! { #s }
    }

    fn ret_suffix(&self) -> TokenStream2 {
        let s = match self {
            GetterMethod::Main { copy: true } => "",
            GetterMethod::Main { copy: false } => ".clone()",
            GetterMethod::AsRef => "",
            GetterMethod::AsMut => "",
        };
        quote! { #s }
    }
}

impl GetterDerive {
    pub fn getter_fn_ident(
        &self,
        method: GetterMethod,
        field_name: Option<&Ident>,
        span: Span,
    ) -> Result<Ident> {
        let base_string = self
            .base
            .as_ref()
            .map(LitStr::value)
            .or(field_name.map(Ident::to_string))
            .ok_or(Error::new(
                span,
                "Unnamed fields must be equipped with `#[getter(base_name = \"name\"]` attribute",
            ))?;

        let name_lit = match method {
            GetterMethod::Main { .. } => &self.main,
            GetterMethod::AsRef => &self.as_ref,
            GetterMethod::AsMut => &self.as_mut,
        };

        let s = format!("{}{}{}", self.prefix.value(), base_string, name_lit.value());

        Ok(Ident::new(&s, span))
    }

    pub fn getter_fn_doc(
        &self,
        method: GetterMethod,
        struct_name: &Ident,
        field_name: Option<&Ident>,
        field_index: usize,
        field_doc: Option<&Attribute>,
    ) -> TokenStream2 {
        let fn_doc = format!(
            "Method {} [`{}::{}`] field.\n",
            method.doc_phrase(),
            struct_name,
            field_name
                .map(Ident::to_string)
                .unwrap_or_else(|| field_index.to_string())
        );

        quote! {
            #[doc = #fn_doc]
            #[doc = #field_doc]
        }
    }
}

fn derive_struct_impl(
    data: DataStruct,
    struct_name: &Ident,
    global_param: ParametrizedAttr,
    impl_generics: ImplGenerics,
    ty_generics: TypeGenerics,
    where_clause: Option<&WhereClause>,
) -> Result<TokenStream2> {
    let mut methods = Vec::with_capacity(data.fields.len());
    match data.fields {
        Fields::Named(ref fields) => {
            for (index, field) in fields.named.iter().enumerate() {
                methods.extend(derive_field_methods(
                    field,
                    index,
                    struct_name,
                    &global_param,
                )?)
            }
        }
        Fields::Unnamed(_) => Err(Error::new(
            Span::call_site(),
            "Deriving getters is not supported for tuple-bases structs",
        ))?,
        Fields::Unit => Err(Error::new(
            Span::call_site(),
            "Deriving getters is meaningless for unit structs",
        ))?,
    };

    Ok(quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #( #methods )*
        }
    })
}

fn derive_field_methods(
    field: &Field,
    index: usize,
    struct_name: &Ident,
    global_param: &ParametrizedAttr,
) -> Result<Vec<TokenStream2>> {
    let mut local_param = ParametrizedAttr::with("getter", &field.attrs)?;

    // First, test individual attribute
    let _ = GetterDerive::try_from(&mut local_param, true)?;
    // Second, combine global and local together
    let getter = GetterDerive::try_from(&mut global_param.clone().merged(local_param)?, false)?;

    if getter.skip {
        return Ok(Vec::new());
    }

    let field_name = field.ident.as_ref();
    let ty = &field.ty;
    let doc = field.attrs.iter().find(|a| a.path.is_ident("doc"));

    let mut res = Vec::with_capacity(3);
    for method in &GetterMethod::all(getter.copy) {
        let fn_name = getter.getter_fn_ident(*method, field_name, field.span())?;
        let fn_doc = getter.getter_fn_doc(*method, struct_name, field_name, index, doc);
        let ret_prefix = method.ret_prefix();
        let ret_suffix = method.ret_suffix();

        res.push(quote_spanned! { field.span() =>
            #fn_doc
            #[inline]
            pub fn #fn_name(&self) -> #ret_prefix #ty {
                #ret_prefix self.#field_name#ret_suffix
            }
        })
    }

    Ok(res)
}
