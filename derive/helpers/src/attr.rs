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

use std::hash::Hash;
use std::fmt::{Debug};
use std::collections::HashMap;
use syn::{Type, Path, Attribute, Meta, MetaList, MetaNameValue, NestedMeta, Lit, LitInt};
use proc_macro2::{Ident, Span};

use crate::Error;

/// Structure representing internal structure of collected instances of a proc
/// macro attribute having some specific name (accessible via [`Attr::name()`]).
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Attr {
    /// Attribute of `#[attr]` or `#[attr = value]` form, which, aside from the
    /// case where `value` is a string literal, may have only a single
    /// occurrence (string literals are concatenated into a single value like
    /// rust compiler does for `#[doc = "..."]` attributes).
    Singular(SingularAttr),

    /// Parametrized attribute in form of `#[attr(...)]`, where parameters are
    /// gathered from all attribute occurrences.
    Parametrized(ParametrizedAttr),
}

/// Structure describing a procedural macro attribute with an optional value.
/// The means that if one has something like `#[name1]`, `#[name2 = "value"]`,
/// `#[name3 = ::std::path::PathBuf)]` than `name1`, `name2 = "value"`, and
/// `name3 = ::std::path::PathBuf` are three different attributes  which can be
/// parsed and represented by the [`SingularAttr`] structure.
///
/// NB: For `#[attr(arg1, arg2 = value)]` style of proc macros use
/// [`ParametrizedAttr`] structure. If you need to support both use [`Attr`]
/// enum.
///
/// Internally the structure is composed of the `name` and `value` fields,
/// where name is always a [`Ident`] (corresponding `name1`, `name2`, `name3`
/// from the sample above) and `value` is an optional literal [`Lit`], with
/// corresponding cases of `None`,
/// `Some(`[`AttrArgValue::Lit`]`(`[`Lit::Str`]`(`[`LitStr`]`)))`, and
/// `Some(`[`AttrArgValue::Type`]`(`[`Type::Path`]`(`[`Path`]`)))`.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SingularAttr {
    /// Optional attribute argument path part; for instance in
    /// `#[my(name = value)]` or in `#[name = value]` this is a `name` part
    pub name: Ident,

    /// Optional attribute argument value part; for instance in
    /// `#[name = value]` this is a `value` part
    pub value: Option<ArgValue>,
}

/// Representation for all allowed forms of `#[attr(...)]` attribute.
/// If attribute has a multiple occurrences they are all assembled into a single
/// list. Repeated named arguments are not allowed and result in errors.
///
/// For situations like in `#[attr("string literal")]`, [`ParametrizedAttr`]
/// will have a `name` field set to `attr`, `literal` field set to
/// `Lit::LitStr(LitStr("string literal"))`, `args` will be an empty `HashSet`
/// and `paths` will be represented by an empty vector.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ParametrizedAttr {
    /// Attribute name - `attr` part of `#[attr(...)]`
    pub name: Ident,

    /// All attribute arguments that have form of `#[attr(ident = "literal")]`
    /// or `#[attr(ident = TypeName)]` mapped to their name identifiers
    pub args: HashMap<Ident, ArgValue>,

    /// All attribute arguments that are paths or identifiers without any
    /// specific value, like `#[attr(std::io::Error, crate, super::SomeType)]`.
    pub paths: Vec<Path>,

    /// Unnamed integer literals found within attribute arguments
    pub integers: Vec<LitInt>,

    /// Unnamed literal value found in the list of attribute arguments.
    /// If multiple literals are found they must be a string literals and
    /// are concatenated into a single value, like it is done by the rust
    /// compiler for `#[doc = "..."]` attributes
    pub literal: Option<Lit>,
}

/// Value for attribute or attribute argument, i.e. for `#[attr = value]` and
/// `#[attr(arg = value)]` this is the `value` part of the attribute. Can be
/// either a single literal or a single valid rust type name
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ArgValue {
    /// Attribute value represented by a literal
    Lit(Lit),

    /// Attribute value represented by a type name
    Type(Type),
}

/// Structure requirements for parametrized attribute
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AttrReq {
    /// Specifies all named arguments and which requirements they must meet
    pub args: HashMap<Ident, ValueReq<ArgValue>>,

    /// Specifies whether path arguments are allowed and with which
    /// requirements.
    pub paths: ListReq<Path>,

    /// Whether integer literals are allowed as an attribute argument and, if
    /// yes, with which requirements
    pub integers: ListReq<LitInt>,

    /// Which other literals are allowed and which requirements should apply.
    ///
    /// NB: Non-string and non-integer literals may be always present only once.
    pub literal: (LitReq, ValueReq<Lit>),
}

/// Requirements for attribute or named argument value presence
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ValueReq<T>
where
    T: Clone + Eq + PartialEq + Hash + Debug,
{
    /// Argument or an attribute must explicitly hold a value
    Required,

    /// Argument or an attribute must hold a value; if the value is not present
    /// it will be substituted for the default value provided as a `T` field.
    Default(T),

    /// Argument or an attribute may or may not hold a value
    Optional,

    /// Argument or an attribute must not a value
    Prohibited,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ListReq<T>
where
    T: Clone + Eq + PartialEq + Hash + Debug,
{
    NoneOrMore,
    OneOrMore,
    Default(T),
    Deny,
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum LitReq {
    StringLiteral,
    ByteLiteral,
    CharLiteral,
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
    Verbatim,
}

impl Attr {
    #[inline]
    pub fn singular_or_err(self) -> Result<SingularAttr, Error> {
        match self {
            Attr::Singular(attr) => Ok(attr),
            Attr::Parametrized(attr) => Err(Error::SingularAttrRequired(attr.name)),
        }
    }

    #[inline]
    pub fn parametrized_or_err(self) -> Result<ParametrizedAttr, Error> {
        match self {
            Attr::Singular(attr) => Err(Error::ParametrizedAttrRequired(attr.name)),
            Attr::Parametrized(attr) => Ok(attr),
        }
    }

    #[inline]
    pub fn name(&self) -> Ident {
        match self {
            Attr::Singular(attr) => attr.name.clone(),
            Attr::Parametrized(attr) => attr.name.clone(),
        }
    }

    #[inline]
    pub fn value(&self) -> Result<ArgValue, Error> {
        match self {
            Attr::Singular(attr) => attr.value(),
            Attr::Parametrized(attr) => Err(Error::ParametrizedAttrHasNoValue(attr.name.clone())),
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
        let ident = attr
            .path
            .get_ident()
            .cloned()
            .ok_or(Error::ArgNameMustBeIdent(attr.path.clone()))?;
        return match attr.parse_meta()? {
            // `#[attr::path]`
            // Probably unreachable since it is filtered at the level of
            // ident computation above
            Meta::Path(_) => Ok(Attr::Singular(SingularAttr::with_name(ident))),
            // `#[ident = lit]`
            Meta::NameValue(MetaNameValue { lit, .. }) => {
                Ok(Attr::Singular(SingularAttr::with_named_literal(ident, lit)))
            }
            // `#[ident(...)]`
            Meta::List(MetaList { nested, .. }) => nested
                .into_iter()
                .map(ParametrizedAttr::with_nested)
                .fold(Ok(ParametrizedAttr::with_name(ident)), |res, arg| {
                    res.and_then(|attr| arg.and_then(|arg| attr.merged(arg)))
                })
                .map(|attr| Attr::Parametrized(attr)),
        };
    }
}

impl SingularAttr {
    #[inline]
    pub fn with_name(name: Ident) -> Self {
        Self { name, value: None }
    }

    #[inline]
    pub fn with_named_literal(name: Ident, lit: Lit) -> Self {
        Self {
            name,
            value: Some(ArgValue::Lit(lit)),
        }
    }

    #[inline]
    pub fn value(&self) -> Result<ArgValue, Error> {
        self.value.ok_or(Error::ArgValueRequired(self.name.clone()))
    }

    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        self.value()?.literal_value()
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        self.value()?.type_value()
    }

    pub fn merge(&mut self, attr: &Attribute) -> Result<(), Error> {
        // TODO: Implement
    }

    pub fn check<T>(&self, req: ValueReq<T>) -> Result<(), Error>
    where
        T: Clone + Eq + PartialEq + Hash + Debug,
    {
        // TODO: Implement
    }

    #[inline]
    pub fn checked<T>(self, req: ValueReq<T>) -> Result<Self, Error>
    where
        T: Clone + Eq + PartialEq + Hash + Debug,
    {
        self.check(req)?;
        Ok(self)
    }
}

impl ParametrizedAttr {
    #[inline]
    pub fn with_name(name: Ident) -> Self {
        Self {
            name,
            args: Default::default(),
            paths: vec![],
            integers: vec![],
            literal: None,
        }
    }

    pub fn with_nested(nested: NestedMeta) -> Result<Self, Error> {
        /*for arg in nested {
            match arg {
                // `#[ident("literal")]`
                // here we concatenate all literals
                NestedMeta::Lit(lit) => {
                    let new = SingularAttr::with_name(lit);
                }
                NestedMeta::Meta(meta) => match meta {
                    // `#[ident(arg::path, ...)]`
                    Meta::Path(_) => {}

                    // `#[ident(arg(...), ...)]`
                    Meta::List(list) => return Err(Error::NestedListsNotSupported(ident, list)),

                    // `#[ident("literal", ...)]`
                    Meta::NameValue(_) => {}
                },
            };
        }*/
    }

    pub fn merge(&mut self, attr: &Attribute) -> Result<(), Error> {
        // TODO: Implement
    }

    pub fn check(&self, req: AttrReq) -> Result<(), Error> {
        // TODO: Implement
    }

    #[inline]
    pub fn checked(self, req: AttrReq) -> Result<Self, Error> {
        self.check(req)?;
        Ok(self)
    }
}

impl ArgValue {
    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        match self {
            ArgValue::Lit(lit) => Ok(lit.clone()),
            ArgValue::Type(_) => Err(Error::ArgValueMustBeLiteral),
        }
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        match self {
            ArgValue::Lit(lit) => Err(Error::ArgValueMustBeType),
            ArgValue::Type(ty) => Ok(ty.clone()),
        }
    }
}

#[doc(hide)]
pub trait ExtractAttr {
    #[doc(hide)]
    fn filter_named(self, name: &str) -> Self;

    #[doc(hide)]
    fn singular_attr<T>(self, name: &str, req: ValueReq<T>) -> Result<SingularAttr, Error>
    where
        T: Clone + Eq + PartialEq + Hash + Debug;

    #[doc(hide)]
    fn parametrized_attr(self, name: &str, req: AttrReq) -> Result<ParametrizedAttr, Error>;
}

impl<'a, T> ExtractAttr for T
where
    T: IntoIterator<Item = &'a Attribute>,
{
    /// Filters iterator leaving only attributes with string representation of
    /// its identity-based name matching `name` argument.
    fn filter_named(self, name: &str) -> Self {
        self.into_iter().filter(|attr| attr.path.is_ident(name))
    }

    /// Returns a [`SingularAttr`] which structure must fulfill the provided
    /// requirements - or fails with a [`Error`] otherwise. For more information
    /// check [`ValueReq`] requirements info.
    fn singular_attr<V>(self, name: &str, req: ValueReq<V>) -> Result<SingularAttr, Error>
    where
        V: Clone + Eq + PartialEq + Hash + Debug,
    {
        let mut attr = SingularAttr::with_name(Ident::new(name, Span::call_site()));
        for entries in self.filter_named(name) {
            attr.merge(entries);
        }
        attr.checked(req)
    }

    /// Returns a [`ParametrizedAttr`] which structure must fulfill the provided
    /// requirements - or fails with a [`Error`] otherwise. For more information
    /// check [`AttrReq`] requirements info.
    fn parametrized_attr(self, name: &str, req: AttrReq) -> Result<ParametrizedAttr, Error> {
        let mut attr = ParametrizedAttr::with_name(Ident::new(name, Span::call_site()));
        for entries in self.filter_named(name) {
            attr.merge(entries);
        }
        attr.checked(req)
    }
}
