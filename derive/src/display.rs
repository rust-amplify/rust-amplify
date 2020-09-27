// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Elichai Turkel <elichai.turkel@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use syn::export::{Span, ToTokens, TokenStream2};
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields,
    Ident, Index, Lit, LitStr, Member, Meta, MetaList, MetaNameValue,
    NestedMeta, Path, PathSegment, Result, Type, TypeSlice,
};

use crate::traits::IntoFields;
use crate::util::attr_named_value;

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    match input.data {
        Data::Struct(ref data) => inner_struct(&input, data),
        Data::Enum(ref data) => inner_enum(&input, data),
        //strict_encode_inner_enum(&input, &data),
        Data::Union(ref data) => unimplemented!(),
    }
}

const NAME: &'static str = "display";
const EXAMPLE: &'static str =
    r#"#[display = "Format {} string" | Trait | Type::function]"#;

fn inner_struct(
    input: &DeriveInput,
    data: &DataStruct,
) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();
    let ident_name = &input.ident;

    let attr = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident(NAME))
        .ok_or(Error::new(
            input.span(),
            format!(
                "Deriving `Display`: required attribute `{}` is missing.\n{}",
                NAME, EXAMPLE
            ),
        ))?;

    let stream = display_attr(attr, &data.fields)?;

    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #stream
            }
        }
    })
}

fn inner_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();
    let ident_name = &input.ident;
    let mut display = vec![];

    let stream = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident(NAME))
        .map(|attr| display_attr(attr, &Fields::Unit))
        .map_or(Ok(None), |r| r.map(Some))?;

    for v in &data.variants {
        let type_name = &v.ident;
        let type_str = format!("{}", type_name);

        let display_str = format_from_attrs(&v.attrs, v.span())?;

        if stream.is_some() && display_str.is_some() {
            Err(attr_err!(v.span(), "attribute can't be used for enum and enum variant at the same time", NAME, EXAMPLE))?;
        }

        match (&v.fields, &display_str) {
            (Fields::Named(_), None) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name { .. } => f.write_str(concat!(#type_str, " { .. }")),
                });
            }
            (Fields::Unnamed(_), None) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name(..) => f.write_str(concat!(#type_str, "(..)")),
                });
            }
            (Fields::Unit, None) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
            (Fields::Named(fields), Some(display_str)) => {
                let idents = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name { #( #idents, )* } => write!(f, #display_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unnamed(fields), Some(display_str)) => {
                let idents = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("_{}", i), v.span()))
                    .collect::<Vec<_>>();
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name ( #( #idents, )* ) => write!(f, #display_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unit, Some(display_str)) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#display_str),
                });
            }
        }
    }

    Ok(match stream {
        Some(stream) => stream,
        None => quote! {
            impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        #( #display )*
                    }
                }
            }
        },
    })
}

fn display_attr(attr: &Attribute, fields: &Fields) -> Result<TokenStream2> {
    let stream = match attr.parse_meta()? {
        Meta::List(list) => {
            if list.nested.len() > 1 {
                Err(attr_err!(attr, NAME, "too many arguments", EXAMPLE))?
            }
            let arg = list.nested.first().ok_or(attr_err!(
                attr.span(),
                NAME,
                "argument is required",
                EXAMPLE
            ))?;
            match arg {
                NestedMeta::Meta(Meta::Path(path)) => {
                    display_using_path(path, arg.span())?
                }
                NestedMeta::Lit(Lit::Str(format)) => dislay_with_format(&fields, format, arg.span()),
                _ => Err(attr_err!(attr.span(), NAME,
                    "argument must be either format string literal, trait or function name",
                    EXAMPLE
                ))?
            }
        }
        Meta::Path(_) => {
            Err(attr_err!(attr.span(), NAME, "must provide value", EXAMPLE))?
        }
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(format),
            ..
        }) => dislay_with_format(&fields, &format, attr.span()),
        _ => Err(attr_err!(
            attr.span(),
            NAME,
            "format must be a string literal",
            EXAMPLE
        ))?,
    };

    Ok(stream)
}

fn display_using_path(path: &Path, span: Span) -> Result<TokenStream2> {
    Ok(match path.segments.first() {
        None => Err(attr_err!(
            span,
            NAME,
            "must contain at least one identifier",
            EXAMPLE
        ))?,
        Some(PathSegment { ident, .. }) if ident == "Debug" => {
            quote_spanned! { span =>
                ::std::fmt::Debug::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "Octal" => {
            quote_spanned! { span =>
                ::std::fmt::Octal::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "Binary" => {
            quote_spanned! { span =>
                ::std::fmt::Binary::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "Pointer" => {
            quote_spanned! { span =>
                ::std::fmt::Pointer::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "LowerHex" => {
            quote_spanned! { span =>
                ::std::fmt::LowerHex::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "UpperHex" => {
            quote_spanned! { span =>
                ::std::fmt::UpperHex::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "LowerExp" => {
            quote_spanned! { span =>
                ::std::fmt::LowerExp::fmt(&self, &mut f)
            }
        }
        Some(PathSegment { ident, .. }) if ident == "UpperExp" => {
            quote_spanned! { span =>
                ::std::fmt::UpperExp::fmt(&self, &mut f)
            }
        }
        _ => {
            quote_spanned! { span =>
                f.write_str(& #path (&self))
            }
        }
    })
}

fn dislay_with_format(
    fields: &Fields,
    format: &LitStr,
    span: Span,
) -> TokenStream2 {
    match fields {
        // Format string
        Fields::Named(fields) => {
            let idents = fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect::<Vec<_>>();
            quote_spanned! { span =>
                write!(f, #format, #( #idents = self.#idents, )* )
            }
        }
        Fields::Unnamed(fields) => {
            let idents = (0..fields.unnamed.len())
                .map(|i| Ident::new(&format!("_{}", i), span))
                .collect::<Vec<_>>();
            let selves = (0..fields.unnamed.len())
                .map(|i| Ident::new(&format!("self.{}", i), span))
                .collect::<Vec<_>>();
            quote_spanned! { span =>
                write!(f, #format, #( #idents = #selves, )* )
            }
        }
        Fields::Unit => {
            quote_spanned! { span =>
                f.write(#format)
            }
        }
    }
}

fn format_from_attrs<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
    span: Span,
) -> Result<Option<LitStr>> {
    macro_rules! err {
        ( $msg:literal ) => {
            Err(attr_err!(span, NAME, $msg, EXAMPLE))?
        };
    }

    let res = match attrs
        .into_iter()
        .find(|attr| attr.path.is_ident(NAME))
        .map(|attr| attr.parse_meta())
        .map_or(Ok(None), |r| r.map(Some))?
    {
        Some(Meta::List(list)) => {
            if list.nested.len() > 1 {
                err!("too many arguments")
            }
            match list.nested.first() {
                Some(NestedMeta::Lit(Lit::Str(format))) => Some(format.clone()),
                Some(_) => err!("argument must be a string literal"),
                None => err!("argument is required"),
            }
        }
        Some(Meta::NameValue(MetaNameValue {
            lit: Lit::Str(format),
            ..
        })) => Some(format),
        Some(_) => err!("argument must be a string literal"),
        None => None,
    };
    Ok(res)
}
