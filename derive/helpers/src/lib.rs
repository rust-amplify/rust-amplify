// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2021 by
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

//! Amplifying Rust language capabilities: helper functions for creating proc
//! macro libraries
//!
//! # Examples
//!
//! `#[name]` - single form
//! `#[name = "literal"]` - optional single value
//! `#[name = TypeName]`
//! `#[name("literal", TypeName)]` - list of arguments

#![recursion_limit = "256"]
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    missing_docs,
    dead_code,
    warnings
)]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use std::collections::HashMap;
use syn::{Type, Path, Attribute, Meta, MetaList, MetaNameValue, NestedMeta};
use proc_macro2::Ident;

pub enum Error {
    /// Attribute argument must be a path identifier like `#[attr(std::io)]`
    /// or `#[attr = std::io]`
    #[display("attribute argument must be a path identifier")]
    ArgMustBePath,

    /// Attribute or attribute argument must has a name
    #[display("attribute argument name is required")]
    ArgNameRequired,

    /// Attribute or attribute argument name (in form of `#[attr(arg = ...)]`)
    /// must be an identifier (like `arg`) and not a path (like `std::io`)
    #[display("attribute arguments must be identifiers, not paths like {0}")]
    ArgNameMustBeIdent(Path),

    /// Attribute or attribute argument must has a value:
    /// `#[attr(arg = value)]`
    #[display("attribute or attribute argument value is required for {0}")]
    ArgValueRequired(Ident),

    /// Parametrized attribute argument must have a literal value (string,
    /// integer etc): `#[attr(arg = "value")]` or `#[arg = 4]`
    #[display("attribute value for {0} must be a literal (string, int etc)")]
    ArgValueMustBeLiteral(Ident),

    /// Parametrized attribute argument must be a valid type name:
    /// `#[attr(arg = u8)]` or `#[arg = String]`
    #[display("attribute value for {0} must be a valid type name")]
    ArgValueMustBeType(Ident),

    /// Parametrized attribute (in form of `#[attr(arg1 = ...)]`) does not
    /// have a single value
    #[display("attribute {0} must be in a `#[{0} = ...]` form")]
    ParametrizedAttrHasNoValue(Ident),
}

/// Structure describing a procedural macro attribute with optional.
/// The means that if one has something like
/// `#[name1]`, `#[name2 = "value"]`, `#[name3 = ::std::path::PathBuf)]`
/// than `name1`, `name2 = "value"`,
/// `::std::path::PathBuf` and `name3 = u8` are three different attribute
/// arguments which can be parsed and represented by the [`AttrArg`] structure.
///
/// Internally the structure is composed of the `name` and `value` fields, where
/// name is always a [`Path`] (corresponding `name1`, `name2`, `name3`
/// `::std::path::PathBuf` from the sample above) and `value` is an optional
/// literal [`Lit`], with corresponding cases of `None`,
/// `Some(`[`AttrArgValue::Lit`]`(`[`Lit::Str`]`(`[`LitStr`]`)))`,
/// `Some(`[`AttrArgValue::Type`]`(`[`Type::Path`]`(`[`Path`]`)))` and `None`.
///
/// For situations like in `#[attr("string literal")]`, `attr` will have a
/// single [`AttrArg`] with `path` set to `None`
pub struct SingularAttr {
    /// Optional attribute argument path part; for instance in
    /// `#[my(name = value)]` or in `#[name = value]` this is a `name` part
    pub path: Option<Path>,

    /// Optional attribute argument value part; for instance in
    /// `#[my(name = value)]` or in `#[name = value]` this is a `value` part
    pub value: Option<AttrArgValue>,
}

impl SingularAttr {
    #[inline]
    pub fn with_ident(ident: Ident) -> Self {
        Self {
            path: Some(Path::from(ident)),
            value: None,
        }
    }

    #[inline]
    pub fn with_literal(lit: Lit) -> Self {
        Self {
            path: None,
            value: Some(AttrArgValue::Lit(lit)),
        }
    }

    #[inline]
    pub fn with_named_literal(ident: Ident, lit: Lit) -> Self {
        Self {
            path: Some(Path::from(ident)),
            value: Some(AttrArgValue::Lit(lit)),
        }
    }

    #[inline]
    pub fn name(&self) -> Result<Ident, Error> {
        let path = self.path.ok_or(Error::ArgNameRequired)?.clone();
        path.get_ident()
            .cloned()
            .ok_or(Error::ArgNameMustBeIdent(path))
    }

    #[inline]
    pub fn path(&self) -> Result<Path, Error> {
        self.path.cloned().ok_or(Error::ArgMustBePath)
    }

    #[inline]
    pub fn value(&self) -> Result<AttrArgValue, Error> {
        self.value.ok_or(Error::ArgValueRequired(self.name()?))
    }

    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        self.value()?.literal_value()
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        self.value()?.type_value()
    }
}

pub enum AttrArgValue {
    Lit(Lit),
    Type(Type),
}

impl AttrArgValue {
    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        match self {
            AttrArgValue::Lit(lit) => Ok(lit),
            AttrArgValue::Type(_) => Err(Error::ArgValueMustBeLiteral(self.name()?)),
        }
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        match self {
            AttrArgValue::Lit(lit) => Err(Error::ArgValueMustBeType(self.name()?)),
            AttrArgValue::Type(ty) => Ok(ty),
        }
    }
}

/// Representation for all allowed forms of `#[attr(...)]` attribute.
/// If attribute has a multiple occurrences they are all assembled into a single
/// list. Repeated named arguments are not allowed and result in errors.
pub struct ParametrizedAttr {
    /// Attribute name - `attr` part of `#[attr(...)]`
    pub name: Ident,

    /// All attribute arguments that have form of `#[attr(ident = "literal")]`
    /// or `#[attr(ident = TypeName)]` mapped to their name identifiers
    pub args: HashMap<Ident, AttrArgValue>,

    /// All attribute arguments that are pathes or idents without any specific
    /// value, like `#[attr(std::io::Error, crate, super::SomeType)]`.
    pub paths: Vec<Path>,

    ///
    pub literal: Lit,
}

pub enum Attr {
    Parametrized(ParametrizedAttr),
    Singular(SingularAttr),
}

impl Attr {
    #[inline]
    pub fn name(&self) -> Result<Ident, Error> {
        match self {
            Attr::Parametrized(ident, _) => Ok(ident.clone()),
            Attr::Singular(arg) => arg.name(),
        }
    }

    #[inline]
    pub fn path(&self) -> Result<Path, Error> {
        match self {
            Attr::Parametrized(ident, _) => Ok(ident.into()),
            Attr::Singular(arg) => arg.path(),
        }
    }

    #[inline]
    pub fn value(&self) -> Result<AttrArgValue, Error> {
        match self {
            Attr::Parametrized(ident, ..) => Err(Error::ParametrizedAttrHasNoValue(ident.clone())),
            Attr::Singular(arg) => arg.value(),
        }
    }

    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        self.value()?.literal_value()
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        self.value()?.type_value()
    }

    pub fn from_attribute(attr: Attribute) -> Result<Self, Error> {
        if let Ok(meta) = attr.parse_meta() {
            let ident = attr
                .path
                .get_ident()
                .cloned()
                .ok_or(Error::ArgNameMustBeIdent(attr.path.clone()))?;
            match meta {
                // `#[attr::path]`
                // Probably unreachable since it is filtered at the level of
                // ident computation above
                Meta::Path(_) => return Ok(Attr::Singular(SingularAttr::with_ident(ident))),
                // `#[ident(...)]`
                Meta::List(MetaList { nested, .. }) => {
                    let mut args = vec![];
                    for arg in nested {
                        match arg {
                            NestedMeta::Meta(meta) => match meta {
                                // `#[ident(arg::path, ...)]`
                                Meta::Path(_) => {}

                                // `#[ident(arg(...), ...)]`
                                Meta::List(list) => {
                                    return Err(Error::NestedListsNotSupported(list))
                                }

                                // `#[ident("literal", ...)]`
                                Meta::NameValue(_) => {}
                            },
                            // `#[ident("literal")]`
                            // here we concatenate all literals
                            NestedMeta::Lit(lit) => {
                                let new = SingularAttr::with_literal(lit);
                            }
                        };
                    }
                    return Ok(Attr::Parametrized(ident, args));
                }
                // `#[ident = lit]`
                Meta::NameValue(MetaNameValue { lit, .. }) => {
                    return Ok(Attr::Singular(SingularAttr::with_named_literal(ident, lit)))
                }
            }
        }
        Ok(())
    }
}

pub trait Parse {
    fn parse(self) -> Result<Vec<attr>, Error>;
}

impl<'a, T> Parse for T
where
    T: IntoIterator<Item = &'a Attribute>,
{
    fn parse(self) -> Result<Vec<Attr>, Error> {
        let mut res = Vec::<Attr>::new();
        for attr in self {
            res.push()
        }
        Ok(res)
    }
}
