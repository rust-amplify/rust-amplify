// Rust language amplification library providing multiple generic trait
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

//! Helper functions for composing proc macros using syn library

use core::iter::ExactSizeIterator;
use syn::export::{Span, TokenStream2};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Error, ExprLit, Field, Ident, Lit, LitInt, Meta, MetaNameValue,
    NestedMeta, Result,
};

/// Macro producing [`Result::Err`] with [`syn::Error`] containing span
/// information from `$attr` (first) argument and formatted string describing
/// concrete error (description is taken from `$msg` second macro argument) and
/// providing an example `$example` (third macro argument) of how the macro
/// should be used.
macro_rules! attr_err {
    ($name:expr, $msg:tt, $example:tt) => {
        attr_err!(::syn::export::Span::call_site(), $name, $msg, $example);
    };
    ($attr:expr, $name:expr, $msg:tt, $example:tt) => {
        ::syn::Error::new(
            $attr.span(),
            format!("Attribute {}: {}\nExample use: {}", $name, $msg, $example),
        );
    };
}

/// Parses attributes from `attr` looking for an attribute with identifier
/// `ident`, returning `Ok(None)` if it is not found. If the attribute is found,
/// function parses attribute metadata in the following way:
/// * If metadata can't be parsed with [`syn::Attribute::parse_meta()`] it
///   returns [`Result::Err`]
/// * If metadata are not in `key=value` form [`syn::Meta::NameValue`], returns
///   error [`Result::Err`] with detailed information
/// * If metadata are in `key=value` form, it returns `value` in form of
///   [`Option::Some()`] [`syn::Lit`] literal
pub fn attr_named_value<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
    ident: &str,
    example: &str,
) -> Result<Option<Lit>> {
    for attr in attrs {
        if attr.path.is_ident(ident) {
            return match attr.parse_meta() {
                Ok(meta) => match meta {
                    Meta::Path(path) => {
                        let msg = format!(
                            r#"must have form `{0}="..."`, not just declarative `{0}`"#,
                            path.get_ident().unwrap_or(
                                &path.segments.last().unwrap().ident
                            )
                        );
                        Err(attr_err!(attr, ident, msg, example))
                    }
                    Meta::List(list) => {
                        let msg = format!(
                            r#"must have form `{0}="..."`, not `{0}(...)`"#,
                            list.path.get_ident().unwrap_or(
                                &list.path.segments.last().unwrap().ident
                            )
                        );
                        Err(attr_err!(attr, ident, msg, example))
                    }
                    Meta::NameValue(name_val) => Ok(Some(name_val.lit)),
                },
                Err(_) => Err(attr_err!(attr, ident, "wrong format", example)),
            };
        }
    }

    Ok(None)
}

pub fn attr_list<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
    ident: &str,
    example: &str,
) -> Result<Option<Vec<NestedMeta>>> {
    for attr in attrs {
        if attr.path.is_ident(ident) {
            return match attr.parse_meta() {
                Ok(meta) => match meta {
                    Meta::Path(_) => Err(attr_err!(
                        ident,
                        "unexpected path argument",
                        example
                    )),
                    Meta::List(list) => {
                        Ok(Some(list.nested.into_iter().collect()))
                    }
                    Meta::NameValue(_) => Err(attr_err!(
                        ident,
                        "unexpected name=value argument",
                        example
                    )),
                },
                Err(_) => Err(attr_err!(ident, "wrong format", example)),
            };
        }
    }

    Ok(None)
}

pub fn attr_nested_one_arg(
    mut list: impl ExactSizeIterator<Item = NestedMeta>,
    attr_name: &str,
    example: &str,
) -> Result<Option<Ident>> {
    match list.len() {
        0 => Err(attr_err!(
            attr_name,
            "unexpected absence of argument",
            example
        )),
        1 => match list.next().expect("Core library iterator is broken") {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => Ok(path.get_ident().cloned()),
                _ => Err(attr_err!(
                    attr_name,
                    "unexpected attribute type",
                    example
                )),
            },
            NestedMeta::Lit(_) => Err(attr_err!(
                attr_name,
                "unexpected literal for type identifier is met",
                example
            )),
        },
        _ => Err(attr_err!(
            attr_name,
            "unexpected multiple type identifiers",
            example
        )),
    }
}

pub fn attr_nested_one_named_value(
    mut list: impl ExactSizeIterator<Item = NestedMeta>,
    attr_name: &str,
    example: &str,
) -> Result<MetaNameValue> {
    match list.len() {
        0 => Err(attr_err!(
            attr_name,
            "unexpected absence of argument",
            example
        )),
        1 => match list.next().expect("Core library iterator is broken") {
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(path) => Ok(path),
                _ => Err(attr_err!(
                    attr_name,
                    "unexpected attribute type",
                    example
                )),
            },
            NestedMeta::Lit(_) => Err(attr_err!(
                attr_name,
                "unexpected literal for type identifier is met",
                example
            )),
        },
        _ => Err(attr_err!(
            attr_name,
            "unexpected multiple type identifiers",
            example
        )),
    }
}
