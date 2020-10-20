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
    WithFormat(LitStr, Option<LitStr>),
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
                if list.nested.len() > 2 {
                    err!(span, "too many arguments")
                }
                let mut iter = list.nested.iter();
                let mut res = match iter.next() {
                    Some(NestedMeta::Lit(Lit::Str(format))) => {
                        Some(Self::WithFormat(format.clone(), None))
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
                };
                match iter.next() {
                    Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(alt),
                        ..
                    }))) => {
                        if Some("alt".to_string()) == path.get_ident().map(Ident::to_string) {
                            if let Some(Technique::WithFormat(fmt, _)) = res {
                                res = Some(Technique::WithFormat(fmt, Some(alt.clone())));
                            } else {
                                err!(
                                    span,
                                    "alternative formatting can be given only if \
                                     the first argument is a format string"
                                )
                            }
                        } else {
                            err!(span, "unknown attribute argument")
                        }
                    }
                    None => (),
                    _ => err!(span, "unrecognizable second argument"),
                }
                res
            }
            Some(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(format),
                ..
            })) => Some(Self::WithFormat(format, None)),
            Some(_) => err!(span, "argument must be a string literal"),
            None => None,
        };
        Ok(res)
    }

    pub fn to_fmt(&self, span: Span, alt: bool) -> Option<TokenStream2> {
        match self {
            Self::FromTrait(fmt) => Some(fmt.into_token_stream2(span)),
            Self::FromMethod(path) => Some(quote! {#path}),
            Self::WithFormat(fmt, fmt_alt) => Some(if alt && fmt_alt.is_some() {
                let alt = fmt_alt
                    .as_ref()
                    .expect("we just checked that there are data");
                quote! {#alt}
            } else {
                quote! {#fmt}
            }),
            Self::DocComments => None,
        }
    }

    pub fn into_token_stream2(self, fields: &Fields, span: Span, alt: bool) -> TokenStream2 {
        match self {
            Technique::FromTrait(fmt) => fmt.into_token_stream2(span),
            Technique::FromMethod(path) => quote_spanned! { span =>
                f.write_str(& #path (&self))
            },
            Technique::WithFormat(fmt, fmt_alt) => {
                let format = if alt && fmt_alt.is_some() {
                    let alt = fmt_alt.expect("we just checked that there are data");
                    quote! { #alt }
                } else {
                    quote! { #fmt }
                };
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
                    .map(|i| quote_spanned! { span => self.#i })
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

    let stream = technique
        .clone()
        .into_token_stream2(&data.fields, input.span(), false);
    let stream_alt = technique.into_token_stream2(&data.fields, input.span(), true);

    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                if !f.alternate() {
                    #stream
                } else {
                    #stream_alt
                }
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

        let format = local
            .clone()
            .and_then(|t| t.to_fmt(v.span(), false))
            .or(doc.clone());
        let format_alt = local.and_then(|t| t.to_fmt(v.span(), true)).or(doc);

        fn has_formatters(ident: &Ident, s: String) -> bool {
            let m1 = format!("{}{}:", '{', ident);
            let m2 = format!("{}{}{}", '{', ident, '}');
            s.contains(&m1) || s.contains(&m2)
        }

        match (&v.fields, &format, &format_alt) {
            (Fields::Named(_), None, _) => {
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name { .. } => f.write_str(concat!(#type_str, " { .. }")),
                });
            }
            (Fields::Unnamed(_), None, _) => {
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name(..) => f.write_str(concat!(#type_str, "(..)")),
                });
            }
            (Fields::Unit, None, _) => {
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
            (Fields::Named(fields), Some(format_str), Some(format_alt)) => {
                use_global = false;
                let f = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                let idents = f
                    .clone()
                    .filter(|ident| has_formatters(ident, format_str.to_string()))
                    .collect::<Vec<_>>();
                let idents_alt = f
                    .filter(|ident| has_formatters(ident, format_alt.to_string()))
                    .collect::<Vec<_>>();
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name { #( #idents, )* .. } => {
                        if !f.alternate() {
                            write!(f, #format_str, #( #idents = #idents, )*)
                        } else {
                            write!(f, #format_alt, #( #idents_alt = #idents_alt, )*)
                        }
                    }
                });
            }
            (Fields::Unnamed(fields), Some(format_str), Some(format_alt)) => {
                use_global = false;
                let f = (0..fields.unnamed.len()).map(|i| Ident::new(&format!("_{}", i), v.span()));
                let idents = f
                    .clone()
                    .filter(|ident| has_formatters(ident, format_str.to_string()))
                    .collect::<Vec<_>>();
                let idents_alt = f
                    .filter(|ident| has_formatters(ident, format_alt.to_string()))
                    .collect::<Vec<_>>();
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name ( #( #idents, )* .. ) => {
                        if !f.alternate() {
                            write!(f, #format_str, #( #idents = #idents, )*)
                        } else {
                            write!(f, #format_alt, #( #idents_alt = #idents_alt, )*)
                        }
                    },
                });
            }
            (Fields::Unit, Some(format_str), Some(format_alt)) => {
                use_global = false;
                display.extend(quote_spanned! { v.span() =>
                    Self::#type_name => f.write_str(if !f.alternate() { #format_str } else { #format_alt }),
                });
            }
            _ => unreachable!(),
        }
    }

    let content = match (use_global, global) {
        (false, _) => quote! {
            match self {
                #display
            }
        },
        (true, Some(tenchique)) => {
            let format_str =
                tenchique
                    .clone()
                    .into_token_stream2(&Fields::Unit, input.span(), false);
            let format_alt = tenchique.into_token_stream2(&Fields::Unit, input.span(), true);
            quote! {
                if f.alternate() {
                    #format_alt
                } else {
                    #format_str
                }
            }
        }
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
            .map(|t| (t.to_fmt(field.span(), false), t.to_fmt(field.span(), true)));

        match format {
            None => {
                display.push(quote_spanned! { field.span() =>
                    Self::#type_name => f.write_str(#type_str),
                });
            }
            Some((format_str, format_alt)) => {
                display.push(quote_spanned! { field.span() =>
                    Self::#type_name => f.write_str(if !f.alternate() { #format_str } else { #format_alt }),
                });
            }
        }
    }

    let content = match global {
        Some(tenchique) => {
            let format_str =
                tenchique
                    .clone()
                    .into_token_stream2(&Fields::Unit, input.span(), false);
            let format_alt = tenchique.into_token_stream2(&Fields::Unit, input.span(), true);
            quote! {
                if f.alternate() {
                    #format_alt
                } else {
                    #format_str
                }
            }
        }
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
