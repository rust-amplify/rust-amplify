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
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error,
    Fields, Ident, Lit, LitStr, Meta, MetaNameValue, NestedMeta, Path, Result,
};

const NAME: &'static str = "display";
const EXAMPLE: &'static str =
    r#"#[display("format {} string" | Trait | Type::function)]"#;

macro_rules! err {
    ( $span:expr, $msg:literal ) => {
        Err(attr_err!($span, NAME, $msg, EXAMPLE))?
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FormattingTrait {
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
pub enum Technique {
    FromTrait(FormattingTrait),
    FromMethod(Path),
    WithFormat(LitStr),
}

impl Technique {
    pub fn from_attrs<'a>(
        attrs: impl IntoIterator<Item = &'a Attribute>,
        span: Span,
    ) -> Result<Option<Self>> {
        let res = match attrs
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
                    Some(NestedMeta::Meta(Meta::Path(path))) => Some(
                        FormattingTrait::from_path(path, list.span())?
                            .map_or(Self::FromMethod(path.clone()), |fmt| {
                                Self::FromTrait(fmt)
                            }),
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

    pub fn into_format(self, span: Span) -> Result<LitStr> {
        Ok(match self {
            Self::WithFormat(format) => format,
            _ => err!(
                span,
                "enum variants may be formatted with string literal only"
            ),
        })
    }

    pub fn into_token_stream2(
        self,
        fields: &Fields,
        span: Span,
    ) -> TokenStream2 {
        match self {
            Technique::FromTrait(fmt) => fmt.into_token_stream2(span),
            Technique::FromMethod(path) => quote_spanned! { span =>
                f.write_str(& #path (&self))
            },
            Technique::WithFormat(format) => {
                Self::impl_format(fields, &format, span)
            }
        }
    }

    fn impl_format(
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
}

pub(crate) fn inner(input: DeriveInput) -> Result<TokenStream2> {
    match input.data {
        Data::Struct(ref data) => inner_struct(&input, data),
        Data::Enum(ref data) => inner_enum(&input, data),
        Data::Union(ref data) => inner_union(&input, data),
    }
}

fn inner_struct(
    input: &DeriveInput,
    data: &DataStruct,
) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();
    let ident_name = &input.ident;

    let technique = Technique::from_attrs(&input.attrs, input.span())?.ok_or(
        Error::new(
            input.span(),
            format!(
                "Deriving `Display`: required attribute `{}` is missing.\n{}",
                NAME, EXAMPLE
            ),
        ),
    )?;

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
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();
    let ident_name = &input.ident;
    let mut display = vec![];

    let global = Technique::from_attrs(&input.attrs, input.span())?;

    for v in &data.variants {
        let type_name = &v.ident;
        let type_str = format!("{}", type_name);

        let local = Technique::from_attrs(&v.attrs, v.span())?;

        if global.is_some() {
            if local.is_some() {
                err!(v.span(), "attribute can't be used for enum and enum variant at the same time")
            } else {
                continue;
            }
        }

        let format = local
            .map(|t| t.into_format(v.span()))
            .map_or(Ok(None), |r| r.map(Some))?;

        match (&v.fields, &format) {
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
            (Fields::Named(fields), Some(format_str)) => {
                let idents = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name { #( #idents, )* } => write!(f, #format_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unnamed(fields), Some(format_str)) => {
                let idents = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("_{}", i), v.span()))
                    .collect::<Vec<_>>();
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name ( #( #idents, )* ) => write!(f, #format_str, #( #idents = #idents, )*),
                });
            }
            (Fields::Unit, Some(format_str)) => {
                display.push(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#format_str),
                });
            }
        }
    }

    let content = match global {
        Some(tenchique) => {
            tenchique.into_token_stream2(&Fields::Unit, input.span())
        }
        None => quote! {
            match self {
                #( #display )*
            }
        },
    };
    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #content
            }
        }
    })
}

fn inner_union(input: &DeriveInput, data: &DataUnion) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();
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
            .map(|t| t.into_format(field.span()))
            .map_or(Ok(None), |r| r.map(Some))?;

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
        Some(tenchique) => {
            tenchique.into_token_stream2(&Fields::Unit, input.span())
        }
        None => quote! {
            match self {
                #( #display )*
            }
        },
    };
    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #content
            }
        }
    })
}
