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

use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields, Ident, Lit,
    LitStr, Meta, MetaNameValue, NestedMeta, Path, Result, Index,
};

const NAME: &str = "display";
const EXAMPLE: &str = r#"#[display("format {} string" | Trait | Type::function)]"#;

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
                    "Debug" => Some(FormattingTrait::Debug),
                    "Octal" => Some(FormattingTrait::Octal),
                    "Binary" => Some(FormattingTrait::Binary),
                    "Pointer" => Some(FormattingTrait::Pointer),
                    "LowerHex" => Some(FormattingTrait::LowerHex),
                    "UpperHex" => Some(FormattingTrait::UpperHex),
                    "LowerExp" => Some(FormattingTrait::LowerExp),
                    "UpperExp" => Some(FormattingTrait::UpperExp),
                    _ => None,
                })
            },
        )
    }

    pub fn to_fmt(self, alt: bool) -> TokenStream2 {
        let mut fmt = match self {
            FormattingTrait::Debug => "{:?}",
            FormattingTrait::Octal => "{:o}",
            FormattingTrait::Binary => "{:b}",
            FormattingTrait::Pointer => "{:p}",
            FormattingTrait::LowerHex => "{:x}",
            FormattingTrait::UpperHex => "{:X}",
            FormattingTrait::LowerExp => "{:e}",
            FormattingTrait::UpperExp => "{:E}",
        }
        .to_owned();
        if alt {
            fmt = fmt.replace(':', ":#");
        }
        quote! { #fmt }
    }

    pub fn into_token_stream2(self, span: Span) -> TokenStream2 {
        match self {
            FormattingTrait::Debug => quote_spanned! { span =>
                ::core::fmt::Debug::fmt(self, &mut f)
            },
            FormattingTrait::Octal => quote_spanned! { span =>
                ::core::fmt::Octal::fmt(self, &mut f)
            },
            FormattingTrait::Binary => quote_spanned! { span =>
                ::core::fmt::Binary::fmt(self, &mut f)
            },
            FormattingTrait::Pointer => quote_spanned! { span =>
                ::core::fmt::Pointer::fmt(self, &mut f)
            },
            FormattingTrait::LowerHex => quote_spanned! { span =>
                ::core::fmt::LowerHex::fmt(self, &mut f)
            },
            FormattingTrait::UpperHex => quote_spanned! { span =>
                ::core::fmt::UpperHex::fmt(self, &mut f)
            },
            FormattingTrait::LowerExp => quote_spanned! { span =>
                ::core::fmt::LowerExp::fmt(self, &mut f)
            },
            FormattingTrait::UpperExp => quote_spanned! { span =>
                ::core::fmt::UpperExp::fmt(self, &mut f)
            },
        }
    }
}

#[derive(Clone)]
enum Technique {
    FromTrait(FormattingTrait),
    FromMethod(Path),
    WithFormat(LitStr, Option<LitStr>),
    DocComments(String),
    Inner,
    Lowercase(String),
    Uppercase(String),
}

impl Technique {
    pub fn from_attrs<'a>(
        attrs: impl IntoIterator<Item = &'a Attribute> + Clone,
        span: Span,
    ) -> Result<Option<Self>> {
        let mut res = match attrs
            .clone()
            .into_iter()
            .find(|attr| attr.path.is_ident(NAME))
            .map(|attr| attr.parse_meta())
            .map_or(Ok(None), |r| r.map(Some))?
        {
            Some(Meta::List(list)) => {
                if list.nested.len() > 2 {
                    return Err(attr_err!(span, "too many arguments"));
                }
                let mut iter = list.nested.iter();
                let mut res = match iter.next() {
                    Some(NestedMeta::Lit(Lit::Str(format))) => {
                        Some(Technique::WithFormat(format.clone(), None))
                    }
                    Some(NestedMeta::Meta(Meta::Path(path)))
                        if path.is_ident("doc_comments") || path.is_ident("docs") =>
                    {
                        Some(Technique::DocComments(String::new()))
                    }
                    Some(NestedMeta::Meta(Meta::Path(path))) if path.is_ident("inner") => {
                        Some(Technique::Inner)
                    }
                    Some(NestedMeta::Meta(Meta::Path(path))) if path.is_ident("lowercase") => {
                        Some(Technique::Lowercase(String::new()))
                    }
                    Some(NestedMeta::Meta(Meta::Path(path))) if path.is_ident("uppercase") => {
                        Some(Technique::Uppercase(String::new()))
                    }
                    Some(NestedMeta::Meta(Meta::Path(path))) => Some(
                        FormattingTrait::from_path(path, list.span())?
                            .map_or(Technique::FromMethod(path.clone()), Technique::FromTrait),
                    ),
                    Some(_) => return Err(attr_err!(span, "argument must be a string literal")),
                    None => return Err(attr_err!(span, "argument is required")),
                };
                res = match iter.next() {
                    Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(alt),
                        ..
                    }))) if Some("alt".to_string()) == path.get_ident().map(Ident::to_string) => {
                        if iter.count() > 0 {
                            return Err(attr_err!(span, "excessive arguments"));
                        }
                        match res {
                            Some(Technique::WithFormat(fmt, _)) => {
                                Some(Technique::WithFormat(fmt, Some(alt.clone())))
                            }
                            _ => {
                                return Err(attr_err!(
                                    span,
                                    "alternative formatting can be given only if \
                                 the first argument is a format string"
                                ))
                            }
                        }
                    }
                    None => res,
                    _ => return Err(attr_err!(span, "unrecognizable second argument")),
                };
                res
            }
            Some(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(format),
                ..
            })) => Some(Technique::WithFormat(format, None)),
            Some(_) => return Err(attr_err!(span, "argument must be a string literal")),
            None => None,
        };

        if let Some(r) = res.as_mut() {
            r.apply_docs(attrs)
        }
        if let Some(r) = res.as_mut() {
            r.fix_fmt()
        };

        Ok(res)
    }

    pub fn to_fmt(&self, alt: bool) -> TokenStream2 {
        match self {
            Technique::FromTrait(fmt) => fmt.to_fmt(alt),
            Technique::FromMethod(_) => quote! { "{}" },
            Technique::WithFormat(fmt, fmt_alt) => {
                if alt && fmt_alt.is_some() {
                    let alt = fmt_alt
                        .as_ref()
                        .expect("we just checked that there are data");
                    quote! {#alt}
                } else {
                    quote! {#fmt}
                }
            }
            Technique::DocComments(doc) => quote! { #doc },
            Technique::Inner => {
                if alt {
                    quote! { "{_0:#}" }
                } else {
                    quote! { "{_0}" }
                }
            }
            Technique::Lowercase(fields_fmt) => quote! { #fields_fmt },
            Technique::Uppercase(fields_fmt) => quote! { #fields_fmt },
        }
    }

    #[allow(clippy::unnecessary_unwrap)]
    pub fn into_token_stream2(self, fields: &Fields, span: Span, alt: bool) -> TokenStream2 {
        match self {
            Technique::FromTrait(fmt) => fmt.into_token_stream2(span),
            Technique::FromMethod(path) => quote_spanned! { span =>
                f.write_str(& #path (self))
            },
            Technique::WithFormat(fmt, fmt_alt) => {
                let format = if alt && fmt_alt.is_some() {
                    let alt = fmt_alt.expect("we just checked that there are data");
                    quote_spanned! { span => #alt }
                } else {
                    quote_spanned! { span => #fmt }
                };
                Self::impl_format(fields, &format, span)
            }
            Technique::DocComments(doc) => {
                let format = quote_spanned! { span => #doc };
                Self::impl_format(fields, &format, span)
            }
            Technique::Inner => {
                let format = if alt {
                    quote_spanned! { span => "{_0:#}" }
                } else {
                    quote_spanned! { span => "{_0}" }
                };
                Self::impl_format(fields, &format, span)
            }
            Technique::Lowercase(fields_fmt) => {
                let format = quote_spanned! { span => #fields_fmt };
                Self::impl_format(fields, &format, span)
            }
            Technique::Uppercase(fields_fmt) => {
                let format = quote_spanned! { span => #fields_fmt };
                Self::impl_format(fields, &format, span)
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
                    .map(|i| {
                        let index = Index::from(i);
                        quote_spanned! { span => self.#index }
                    })
                    .collect::<Vec<_>>();
                quote_spanned! { span =>
                    write!(f, #format, #( #idents = #selves, )* )
                }
            }
            Fields::Unit => {
                quote_spanned! { span =>
                    f.write_str(#format)
                }
            }
        }
    }

    fn apply_docs<'a>(&mut self, attrs: impl IntoIterator<Item = &'a Attribute> + Clone) {
        if let Technique::DocComments(ref mut doc) = self {
            for attr in attrs.into_iter().filter(|attr| attr.path.is_ident("doc")) {
                if let Ok(Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(s), ..
                })) = attr.parse_meta()
                {
                    let fragment = s.value().trim().replace("\\n", "\n");
                    if fragment.is_empty() || fragment == "\n" {
                        doc.push('\n');
                    } else {
                        doc.push_str(&fragment);
                        doc.push(' ');
                    }
                }
            }
            *doc = doc.trim().replace(" \n", "\n");
        }
    }

    fn apply_case(&mut self, type_str: &str, fields: &Fields) {
        let (type_str_cased, fields_fmt) = match self {
            Technique::Lowercase(ref mut f) => (type_str.to_lowercase(), f),
            Technique::Uppercase(ref mut f) => (type_str.to_uppercase(), f),
            _ => unreachable!(),
        };
        *fields_fmt = match fields {
            Fields::Named(f) => {
                let idents = f
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();
                let inner = idents
                    .iter()
                    .map(|ident| format!("{}: {{{0}}}", ident.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {{{{ {} }}}}", type_str_cased, inner)
            }
            Fields::Unnamed(f) => {
                let inner = (0..f.unnamed.len())
                    .map(|i| format!("{{_{}}}", i.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", type_str_cased, inner)
            }
            Fields::Unit => type_str_cased,
        };
    }

    fn fix_fmt(&mut self) {
        fn fix(s: &str) -> String {
            s.replace("{0", "{_0")
                .replace("{1", "{_1")
                .replace("{2", "{_2")
                .replace("{3", "{_3")
                .replace("{4", "{_4")
                .replace("{5", "{_5")
                .replace("{6", "{_6")
                .replace("{7", "{_7")
                .replace("{8", "{_8")
                .replace("{9", "{_9")
        }

        if let Technique::WithFormat(fmt, x) = self {
            *self = Technique::WithFormat(
                LitStr::new(&fix(&fmt.value()), Span::call_site()),
                x.clone(),
            );
        }
        if let Technique::WithFormat(x, Some(fmt)) = self {
            *self = Technique::WithFormat(
                x.clone(),
                Some(LitStr::new(&fix(&fmt.value()), Span::call_site())),
            );
        }
        if let Technique::DocComments(fmt) = self {
            *self = Technique::DocComments(fix(fmt))
        }
    }
}

fn has_formatters(ident: impl ToString, s: &str) -> bool {
    let m1 = format!("{}{}:", '{', ident.to_string());
    let m2 = format!("{}{}{}", '{', ident.to_string(), '}');
    s.contains(&m1) || s.contains(&m2)
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

    let mut technique = Technique::from_attrs(&input.attrs, input.span())?.ok_or_else(|| {
        Error::new(
            input.span(),
            format!(
                "Deriving `Display`: required attribute `{}` is missing.\n{}",
                NAME, EXAMPLE
            ),
        )
    })?;
    technique.apply_docs(&input.attrs);

    let tokens_fmt = technique.to_fmt(false);
    let tokens_alt = technique.to_fmt(true);
    let str_fmt = tokens_fmt.to_string();
    let str_alt = tokens_alt.to_string();

    let display = match (&data.fields, &technique) {
        (_, Technique::FromTrait(_)) | (_, Technique::FromMethod(_)) => technique
            .clone()
            .into_token_stream2(&data.fields, input.span(), false),
        (Fields::Named(fields), Technique::Inner) => {
            if fields.named.len() != 1 {
                return Err(attr_err!(
                    fields.span(),
                    "display(inner) requires only single field in the structure"
                ));
            }
            let field = fields
                .named
                .first()
                .expect("we just checked that there is a single field")
                .ident
                .as_ref()
                .expect("named fields always have ident with the name");
            quote_spanned! { field.span() =>
                write!(f, #tokens_fmt, _0 = self.#field)
            }
        }
        (Fields::Named(fields), _) => {
            let f = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let idents = f
                .clone()
                .filter(|ident| has_formatters(ident, &str_fmt))
                .collect::<Vec<_>>();
            if str_fmt == str_alt {
                quote_spanned! { fields.span() =>
                    write!(f, #tokens_fmt, #( #idents = self.#idents, )*)
                }
            } else {
                let idents_alt = f
                    .filter(|ident| has_formatters(ident, &str_alt))
                    .collect::<Vec<_>>();
                if str_fmt != str_alt {
                    quote_spanned! { fields.span() =>
                        if !f.alternate() {
                            write!(f, #tokens_fmt, #( #idents = self.#idents, )*)
                        } else {
                            write!(f, #tokens_alt, #( #idents_alt = self.#idents_alt, )*)
                        }
                    }
                } else {
                    quote_spanned! { fields.span() =>
                        write!(f, #tokens_fmt, #( #idents = self.#idents, )*)
                    }
                }
            }
        }
        (Fields::Unnamed(fields), _) => {
            let f = (0..fields.unnamed.len()).map(Index::from);
            let idents = f
                .clone()
                .filter(|ident| has_formatters(format!("_{}", ident.index), &str_fmt));
            let nums = idents
                .clone()
                .map(|ident| Ident::new(&format!("_{}", ident.index), fields.span()))
                .collect::<Vec<_>>();
            let idents = idents.collect::<Vec<_>>();
            if str_fmt == str_alt {
                quote_spanned! { fields.span() =>
                    write!(f, #tokens_fmt, #( #nums = self.#idents, )*)
                }
            } else {
                let idents_alt =
                    f.filter(|ident| has_formatters(format!("_{}", ident.index), &str_alt));
                let nums_alt = idents_alt
                    .clone()
                    .map(|ident| Ident::new(&format!("_{}", ident.index), fields.span()))
                    .collect::<Vec<_>>();
                let idents_alt = idents_alt.collect::<Vec<_>>();
                if str_fmt != str_alt {
                    quote_spanned! { fields.span() =>
                        if !f.alternate() {
                            write!(f, #tokens_fmt, #( #nums = self.#idents, )*)
                        } else {
                            write!(f, #tokens_alt, #( #nums_alt = self.#idents_alt, )*)
                        }
                    }
                } else {
                    quote_spanned! { fields.span() =>
                        write!(f, #tokens_fmt, #( #nums = self.#idents, )*)
                    }
                }
            }
        }
        (Fields::Unit, _) => {
            if str_fmt == str_alt {
                quote_spanned! { data.fields.span() =>
                    f.write_str(#tokens_fmt)
                }
            } else {
                quote_spanned! { data.fields.span() =>
                    f.write_str(if !f.alternate() { #tokens_fmt } else { #tokens_alt })
                }
            }
        }
    };

    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #ident_name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #display
            }
        }
    })
}

fn inner_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident_name = &input.ident;
    let mut display = TokenStream2::new();

    let global = Technique::from_attrs(&input.attrs, input.span())?;
    // Ancient rust versions do not known about `matches!` macro
    #[allow(clippy::match_like_matches_macro)]
    let mut use_global = match global {
        Some(Technique::Inner) | Some(Technique::Lowercase(_)) | Some(Technique::Uppercase(_)) => {
            false
        }
        _ => true,
    };

    for v in &data.variants {
        let type_name = &v.ident;
        let type_str = format!("{}", type_name);

        let mut local = Technique::from_attrs(&v.attrs, v.span())?;
        let mut parent = global.clone();
        let current = local.as_mut().or_else(|| parent.as_mut());
        let mut current = current
            .map(|r| {
                r.apply_docs(&v.attrs);
                r
            })
            .cloned();

        if local.is_some() {
            use_global = false;
        }

        if let Some(Technique::DocComments(_))
        | Some(Technique::Lowercase(_))
        | Some(Technique::Uppercase(_)) = current
        {
            use_global = false;
            if let Some(t) = current.as_mut() {
                match t {
                    Technique::DocComments(_) => {
                        *t = Technique::DocComments(String::new());
                        t.apply_docs(&v.attrs);
                        t.fix_fmt();
                    }
                    Technique::Lowercase(_) => {
                        *t = Technique::Lowercase(String::new());
                        t.apply_case(&type_str, &v.fields);
                        t.fix_fmt();
                    }
                    Technique::Uppercase(_) => {
                        *t = Technique::Uppercase(String::new());
                        t.apply_case(&type_str, &v.fields);
                        t.fix_fmt();
                    }
                    _ => unreachable!(),
                }
            }
        }

        let tokens_fmt = current.as_ref().map(|t| t.to_fmt(false));
        let tokens_alt = current.as_ref().map(|t| t.to_fmt(true));

        match (&v.fields, &tokens_fmt, &tokens_alt) {
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
            (Fields::Named(fields), Some(tokens_fmt), Some(tokens_alt)) => {
                if let Some(Technique::Inner) = current {
                    if fields.named.len() != 1 {
                        return Err(attr_err!(
                            fields.span(),
                            "display(inner) requires only single field in the structure"
                        ));
                    }
                    let field = fields
                        .named
                        .first()
                        .expect("we just checked that there is a single field")
                        .ident
                        .as_ref()
                        .expect("named fields always have ident with the name");
                    display.extend(quote_spanned! { v.span() =>
                        Self::#type_name { #field, .. } => {
                            write!(f, #tokens_fmt, _0 = #field)
                        }
                    });
                } else if let Some(Technique::FromTrait(tr)) = current {
                    let stream =
                        Technique::FromTrait(tr).into_token_stream2(&v.fields, v.span(), false);
                    display.extend(quote_spanned! { v.span() =>
                        Self::#type_name { .. } => {
                            #stream
                        }
                    })
                } else {
                    let f = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                    let idents = f
                        .clone()
                        .filter(|ident| has_formatters(ident, &tokens_fmt.to_string()))
                        .collect::<Vec<_>>();
                    let idents_alt = f
                        .filter(|ident| has_formatters(ident, &tokens_alt.to_string()))
                        .collect::<Vec<_>>();
                    if tokens_fmt.to_string() != tokens_alt.to_string() {
                        display.extend(quote_spanned! { v.span() =>
                            Self::#type_name { #( #idents, )* .. } if !f.alternate() => {
                                write!(f, #tokens_fmt, #( #idents = #idents, )*)
                            },
                            Self::#type_name { #( #idents, )* .. } => {
                                write!(f, #tokens_alt, #( #idents_alt = #idents_alt, )*)
                            },
                        });
                    } else {
                        display.extend(quote_spanned! { v.span() =>
                            Self::#type_name { #( #idents, )* .. } => {
                                write!(f, #tokens_fmt, #( #idents = #idents, )*)
                            },
                        });
                    }
                }
            }
            (Fields::Unnamed(fields), Some(tokens_fmt), Some(tokens_alt)) => {
                if let Some(Technique::FromTrait(tr)) = current {
                    let stream =
                        Technique::FromTrait(tr).into_token_stream2(&v.fields, v.span(), false);
                    display.extend(quote_spanned! { v.span() =>
                        Self::#type_name(..) => {
                            #stream
                        }
                    })
                } else {
                    let f =
                        (0..fields.unnamed.len()).map(|i| Ident::new(&format!("_{}", i), v.span()));
                    let idents = f
                        .clone()
                        .filter(|ident| has_formatters(ident, &tokens_fmt.to_string()))
                        .collect::<Vec<_>>();
                    let idents_alt = f
                        .filter(|ident| has_formatters(ident, &tokens_alt.to_string()))
                        .collect::<Vec<_>>();
                    if tokens_fmt.to_string() != tokens_alt.to_string() {
                        display.extend(quote_spanned! { v.span() =>
                            Self::#type_name ( #( #idents, )* .. ) if !f.alternate() => {
                                write!(f, #tokens_fmt, #( #idents = #idents, )*)
                            },
                            Self::#type_name ( #( #idents, )* .. ) => {
                                write!(f, #tokens_alt, #( #idents_alt = #idents_alt, )*)
                            },
                        });
                    } else {
                        display.extend(quote_spanned! { v.span() =>
                            Self::#type_name ( #( #idents, )* .. ) => {
                                write!(f, #tokens_fmt, #( #idents = #idents, )*)
                            },
                        });
                    }
                }
            }
            (Fields::Unit, Some(tokens_fmt), Some(tokens_alt)) => {
                if let Some(Technique::Inner) = current {
                    display.extend(quote_spanned! { v.span() =>
                        Self::#type_name => f.write_str(#type_str),
                    });
                } else {
                    display.extend(quote_spanned! { v.span() =>
                        Self::#type_name => f.write_str(if !f.alternate() { #tokens_fmt } else { #tokens_alt }),
                    });
                }
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
            if format_str.to_string() != format_alt.to_string() {
                quote! {
                    if f.alternate() {
                        #format_alt
                    } else {
                        #format_str
                    }
                }
            } else {
                quote! { #format_str }
            }
        }
        _ => unreachable!(),
    };

    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #ident_name #ty_generics #where_clause {
            #![allow(clippy::if_same_then_else)]
            fn fmt(&self, mut f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
            .or_else(|| global.clone())
            .map(|t| (t.to_fmt(false), t.to_fmt(true)));

        match format {
            None => {
                display.push(quote_spanned! { field.span() =>
                    Self::#type_name => #type_str,
                });
            }
            Some((format_str, format_alt)) => {
                display.push(quote_spanned! { field.span() =>
                    Self::#type_name if !f.alternate() => #format_str,
                    Self::#type_name => #format_alt,
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
            if format_str.to_string() != format_alt.to_string() {
                quote! {
                    if f.alternate() {
                        #format_alt
                    } else {
                        #format_str
                    }
                }
            } else {
                quote! { #format_str }
            }
        }
        None => quote! {
            let s = match self {
                #( #display )*
            };
            f.write_str(s)
        },
    };
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #ident_name #ty_generics #where_clause {
            #![allow(clippy::if_same_then_else)]
            fn fmt(&self, mut f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #content
            }
        }
    })
}
