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

use syn::export::{Span, TokenStream2};
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields, Ident, Lit,
    LitStr, Meta, MetaNameValue, NestedMeta, Path, Result,
};

const NAME: &'static str = "display";
const EXAMPLE: &'static str = r#"#[display("format {} string" | Trait | Type::function)]"#;

macro_rules! err {
    ( $span:expr, $msg:literal ) => {
        Err(attr_err!($span, NAME, $msg, EXAMPLE))?
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FormattingTrait {
    Debug,
    Octal,
    Binary,
    Pointer,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
}

impl FormattingTrait {
    pub fn from_path(path: &Path, span: Span) -> Result<Option<Self>> {
        path.segments.first().map_or(
            Err(attr_err!(
                span,
                NAME,
                "must contain at least one identifier",
                EXAMPLE
            )),
            |segment| {
                Ok(match segment.ident.to_string().as_str() {
                    "Debug" => Some(Self::Debug),
                    "Octal" => Some(Self::Octal),
                    "Binary" => Some(Self::Binary),
                    "Pointer" => Some(Self::Pointer),
                    "LowerHex" => Some(Self::LowerHex),
                    "UpperHex" => Some(Self::UpperHex),
                    "LowerExp" => Some(Self::LowerExp),
                    "UpperExp" => Some(Self::UpperExp),
                    _ => None,
                })
            },
        )
    }

    pub fn into_token_stream2(self, span: Span) -> TokenStream2 {
        match self {
            FormattingTrait::Debug => quote_spanned! { span =>
                ::std::fmt::Debug::fmt(&self, &mut f)
            },
            FormattingTrait::Octal => quote_spanned! { span =>
                ::std::fmt::Octal::fmt(&self, &mut f)
            },
            FormattingTrait::Binary => quote_spanned! { span =>
                ::std::fmt::Binary::fmt(&self, &mut f)
            },
            FormattingTrait::Pointer => quote_spanned! { span =>
                ::std::fmt::Pointer::fmt(&self, &mut f)
            },
            FormattingTrait::LowerHex => quote_spanned! { span =>
                ::std::fmt::LowerHex::fmt(&self, &mut f)
            },
            FormattingTrait::UpperHex => quote_spanned! { span =>
                ::std::fmt::UpperHex::fmt(&self, &mut f)
            },
            FormattingTrait::LowerExp => quote_spanned! { span =>
                ::std::fmt::LowerExp::fmt(&self, &mut f)
            },
            FormattingTrait::UpperExp => quote_spanned! { span =>
                ::std::fmt::UpperExp::fmt(&self, &mut f)
            },
        }
    }
}

#[derive(Clone)]
enum Technique {
    FromTrait(FormattingTrait),
    FromMethod(Path),
    WithFormat(LitStr),
    DocComments,
}

impl Technique {
    pub fn from_attrs<'a>(
        attrs: impl IntoIterator<Item = &'a Attribute> + Clone,
        span: Span,
    ) -> Result<Option<Self>> {
        let res = match attrs
            .clone()
            .into_iter()
            .find(|attr| attr.path.is_ident(NAME))
            .map(|attr| attr.parse_meta())
            .map_or(Ok(None), |r| r.map(Some))?
        {
            Some(Meta::List(list)) => {
                if list.nested.len() > 1 {
                    err!(span, "too many arguments")
                }
                match list.nested.first() {
                    Some(NestedMeta::Lit(Lit::Str(format))) => {
                        Some(Self::WithFormat(format.clone()))
                    }
                    Some(NestedMeta::Meta(Meta::Path(path))) if path.is_ident("doc_comments") => {
                        Some(Self::DocComments)
                    }
                    Some(NestedMeta::Meta(Meta::Path(path))) => Some(
                        FormattingTrait::from_path(path, list.span())?
                            .map_or(Self::FromMethod(path.clone()), |fmt| Self::FromTrait(fmt)),
                    ),
                    Some(_) => err!(span, "argument must be a string literal"),
                    None => err!(span, "argument is required"),
                }
            }
            Some(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(format),
                ..
            })) => Some(Self::WithFormat(format)),
            Some(_) => err!(span, "argument must be a string literal"),
            None => None,
        };
        Ok(res)
    }

    pub fn to_fmt(&self, span: Span) -> Option<TokenStream2> {
        match self {
            Self::FromTrait(fmt) => Some(fmt.into_token_stream2(span)),
            Self::FromMethod(path) => Some(quote! {#path}),
            Self::WithFormat(fmt) => Some(quote! {#fmt}),
            Self::DocComments => None,
        }
    }

    pub fn into_token_stream2(self, fields: &Fields, span: Span) -> TokenStream2 {
        match self {
            Technique::FromTrait(fmt) => fmt.into_token_stream2(span),
            Technique::FromMethod(path) => quote_spanned! { span =>
                f.write_str(& #path (&self))
            },
            Technique::WithFormat(format) => {
                let format = quote! { #format };
                Self::impl_format(fields, &format, span)
            }
            Technique::DocComments => {
                quote! {}
            }
        }
    }

    fn impl_format(fields: &Fields, format: &TokenStream2, span: Span) -> TokenStream2 {
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

    fn doc_attr(attrs: &Vec<Attribute>) -> Result<TokenStream2> {
        attrs
            .iter()
            .filter(|attr| attr.path.is_ident("doc"))
            .try_fold(TokenStream2::new(), |stream, attr| {
                match attr.parse_meta() {
                    Ok(Meta::NameValue(MetaNameValue { lit, .. })) => {
                        if stream.is_empty() {
                            Ok(quote! { #lit })
                        } else {
                            Ok(quote! { concat!(#stream, #lit) })
                        }
                    }
                    _ => Err(attr_err!(
                        attr.span(),
                        NAME,
                        "malformed doc attribute",
                        EXAMPLE
                    )),
                }
            })
    }
}

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    match input.data {
        Data::Struct(ref data) => inner_struct(&input, data),
        Data::Enum(ref data) => inner_enum(&input, data),
        Data::Union(ref data) => inner_union(&input, data),
    }
}

fn inner_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;

    let technique = Technique::from_attrs(&input.attrs, input.span())?.ok_or(Error::new(
        input.span(),
        format!(
            "Deriving `Display`: required attribute `{}` is missing.\n{}",
            NAME, EXAMPLE
        ),
    ))?;

    let stream = technique.into_token_stream2(&data.fields, input.span());

    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #stream
            }
        }
    })
}

fn inner_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;
    let mut display = TokenStream2::new();

    let global = Technique::from_attrs(&input.attrs, input.span())?;
    let mut use_global = true;

    for v in &data.variants {
        let type_name = &v.ident;
        let type_str = format!("{}", type_name);

        let local = Technique::from_attrs(&v.attrs, v.span())?;

        let doc = match global {
            Some(Technique::DocComments) => {
                use_global = false;
                Some(Technique::doc_attr(&v.attrs)?)
            }
            _ => None,
        };

        let format = local.and_then(|t| t.to_fmt(v.span())).or(doc);

        match (&v.fields, &format) {
            (Fields::Named(_), None) => {
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name { .. } => f.write_str(concat!(#type_str, " { .. }")),
                });
            }
            (Fields::Unnamed(_), None) => {
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name(..) => f.write_str(concat!(#type_str, "(..)")),
                });
            }
            (Fields::Unit, None) => {
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
            (Fields::Named(fields), Some(format_str)) => {
                use_global = false;
                let idents = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .filter(|ident| {
                        let s = format_str.to_string();
                        let m1 = format!("{}{}:", '{', ident);
                        let m2 = format!("{}{}{}", '{', ident, '}');
                        s.contains(&m1) || s.contains(&m2)
                    })
                    .collect::<Vec<_>>();
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name { #( #idents, )* .. } => write!(f, #format_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unnamed(fields), Some(format_str)) => {
                use_global = false;
                let idents = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("_{}", i), v.span()))
                    .filter(|ident| {
                        let s = format_str.to_string();
                        let m1 = format!("{}{}:", '{', ident);
                        let m2 = format!("{}{}{}", '{', ident, '}');
                        s.contains(&m1) || s.contains(&m2)
                    })
                    .collect::<Vec<_>>();
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name ( #( #idents, )* .. ) => write!(f, #format_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unit, Some(format_str)) => {
                use_global = false;
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#format_str),
                });
            }
        }
    }

    let content = match (use_global, global) {
        (false, _) => quote! {
            match self {
                #display
            }
        },
        (true, Some(tenchique)) => tenchique.into_token_stream2(&Fields::Unit, input.span()),
        _ => unreachable!(),
    };

    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #content
            }
        }
    })
}

fn inner_union(input: &DeriveInput, data: &DataUnion) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;
    let mut display = vec![];

    let global = Technique::from_attrs(&input.attrs, input.span())?;

    for field in &data.fields.named {
        let type_name = field
            .ident
            .clone()
            .expect("named attributes are always named");
        let type_str = format!("{}", type_name);

        let format = Technique::from_attrs(&field.attrs, field.span())?
            .or(global.clone())
            .and_then(|t| t.to_fmt(field.span()));

        match format {
            None => {
                display.push(quote_spanned! { field.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
            Some(format) => {
                display.push(quote_spanned! { field.span() =>
                    Self::#type_name => f.write_str(#format),
                });
            }
        }
    }

    let content = match global {
        Some(tenchique) => tenchique.into_token_stream2(&Fields::Unit, input.span()),
        None => quote! {
            match self {
                #( #display )*
            }
        },
    };
    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #content
            }
        }
    })
}
