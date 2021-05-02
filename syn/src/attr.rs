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
use syn::{Type, Path, Attribute, Meta, MetaList, MetaNameValue, NestedMeta, Lit, LitInt, LitStr};
use syn::spanned::Spanned;

use crate::Error;
use proc_macro2::TokenStream;
use syn::__private::ToTokens;

/// Internal structure representation of a proc macro attribute collected
/// instances having some specific name (accessible via [`Attr::name()`]).
#[derive(Clone)]
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
#[derive(Clone)]
pub struct SingularAttr {
    /// Optional attribute argument path part; for instance in
    /// `#[my(name = value)]` or in `#[name = value]` this is a `name` part
    pub name: String,

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
#[derive(Clone)]
pub struct ParametrizedAttr {
    /// Attribute name - `attr` part of `#[attr(...)]`
    pub name: String,

    /// All attribute arguments that have form of `#[attr(ident = "literal")]`
    /// or `#[attr(ident = TypeName)]` mapped to their name identifiers
    pub args: HashMap<String, ArgValue>,

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
#[derive(Clone)]
pub enum ArgValue {
    /// Attribute value represented by a literal
    Literal(Lit),

    /// Attribute value represented by a type name
    Type(Type),
}

impl ArgValue {
    #[inline]
    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            ArgValue::Literal(lit) => lit.to_token_stream(),
            ArgValue::Type(ty) => ty.to_token_stream(),
        }
    }

    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        match self {
            ArgValue::Literal(lit) => Ok(lit.clone()),
            ArgValue::Type(_) => Err(Error::ArgValueMustBeLiteral),
        }
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        match self {
            ArgValue::Literal(_) => Err(Error::ArgValueMustBeType),
            ArgValue::Type(ty) => Ok(ty.clone()),
        }
    }
}

/// Structure requirements for parametrized attribute
#[derive(Clone)]
pub struct AttrReq {
    /// Specifies all named arguments and which requirements they must meet
    pub args: HashMap<String, ValueReq>,

    /// Specifies whether path arguments are allowed and with which
    /// requirements.
    pub paths: ListOccurrences<Path>,

    /// Whether integer literals are allowed as an attribute argument and, if
    /// yes, with which requirements
    pub integers: ListOccurrences<LitInt>,

    /// Which other literals are allowed and which requirements should apply.
    ///
    /// NB: Non-string and non-integer literals may be always present only once.
    pub literal: (LiteralConstraints, ValueOccurrences),
}

/*
impl AttrReq {
    pub fn with(args: Vec<(String, ValueOccurrences, ValueConstraints)>, paths: Vec<>)
}
 */

/// Requirements for attribute or named argument value presence
#[derive(Clone)]
pub struct ValueReq {
    pub constraints: ValueConstraints,
    pub occurrences: ValueOccurrences,
}

/// Requirements for attribute or named argument value presence
#[derive(Clone)]
pub enum ValueOccurrences {
    /// Argument or an attribute must explicitly hold a value
    Required,

    /// Argument or an attribute must hold a value; if the value is not present
    /// it will be substituted for the default value provided as the inner field
    Default(ArgValue),

    /// Argument or an attribute may or may not hold a value
    Optional,

    /// Argument or an attribute must not hold a value
    Prohibited,
}

impl ValueOccurrences {
    pub fn check(self, value: &mut Option<ArgValue>, attr: String) -> Result<(), Error> {
        let attr = attr.to_string();
        match (self, value) {
            (ValueOccurrences::Required, None) => Err(Error::SingularAttrRequired(attr)),
            (ValueOccurrences::Prohibited, Some(_)) => Err(Error::AttrMustNotHaveValue(attr)),
            (ValueOccurrences::Default(ref val), v) if v.is_none() => {
                *v = Some(val.clone());
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

/// Requirements for a [`ParametrizedAttr`] elements
#[derive(Clone)]
pub enum ListOccurrences<T>
where
    T: Clone,
{
    /// Element may not be present or may be present multiple times
    NoneOrMore,

    /// Element must be present at least once, or may have multiple occurrences
    OneOrMore,

    /// Element must be present exact amount of times
    Default(T),

    /// Element must not be present
    Deny,
}

/// Constrains for attribute value type
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ValueConstraints {
    /// The value must be a literal matching given literal constraints (see
    /// [`ConstrainedLit`])
    Literal(LiteralConstraints),

    /// The value must be of a native rust type matching given type constraints
    /// (see [`ConstrainedType`])
    Type(TypeConstraints),
}

impl ValueConstraints {
    pub fn check(self, value: &ArgValue, attr: String) -> Result<(), Error> {
        match (self, value) {
            (ValueConstraints::Literal(lit), ArgValue::Literal(ref value)) => {
                lit.check(value, attr)
            }
            (ValueConstraints::Type(ty), ArgValue::Type(ref value)) => ty.check(value, attr),
            _ => Err(Error::AttrValueTypeMimatch(attr)),
        }
    }
}

/// Constrains for literal value type
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum LiteralConstraints {
    /// Literal must be a string
    StringLiteral,

    /// Literal must be a byte string
    ByteStrLiteral,

    /// Literal must be a byte (in form of `b'f'`)
    ByteLiteral,

    /// Literal must be a character
    CharLiteral,

    /// Literal must be an integer
    IntLiteral,

    /// Literal must be a float
    FloatLiteral,

    /// Literal must be a boolean
    BoolLiteral,

    /// Literal must be a verbatim form
    Verbatim,
}

impl LiteralConstraints {
    pub fn check(self, lit: &Lit, attr: String) -> Result<(), Error> {
        match (self, lit) {
            (LiteralConstraints::BoolLiteral, Lit::Bool(_))
            | (LiteralConstraints::ByteLiteral, Lit::Byte(_))
            | (LiteralConstraints::ByteStrLiteral, Lit::ByteStr(_))
            | (LiteralConstraints::CharLiteral, Lit::Char(_))
            | (LiteralConstraints::FloatLiteral, Lit::Float(_))
            | (LiteralConstraints::IntLiteral, Lit::Int(_))
            | (LiteralConstraints::StringLiteral, Lit::Str(_))
            | (LiteralConstraints::Verbatim, Lit::Verbatim(_)) => Ok(()),
            _ => Err(Error::AttrValueTypeMimatch(attr)),
        }
    }
}

/// Constrains for the possible types that a Rust value could have.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeConstraints {
    /// A fixed size array type: `[T; n]`.
    Array,

    /// A bare function type: `fn(usize) -> bool`.
    BareFn,

    /// A type contained within invisible delimiters.
    Group,

    /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
    /// a lifetime.
    ImplTrait,

    /// Indication that a type should be inferred by the compiler: `_`.
    Infer,

    /// A macro in the type position.
    Macro,

    /// The never type: `!`.
    Never,

    /// A parenthesized type equivalent to the inner type.
    Paren,

    /// A path like `std::slice::Iter`, optionally qualified with a
    /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
    Path,

    /// A raw pointer type: `*const T` or `*mut T`.
    Ptr,

    /// A reference type: `&'a T` or `&'a mut T`.
    Reference,

    /// A dynamically sized slice type: `[T]`.
    Slice,

    /// A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a
    /// trait or a lifetime.
    TraitObject,

    /// A tuple type: `(A, B, C, String)`.
    Tuple,

    /// Tokens in type position not interpreted by Syn.
    Verbatim,
}

impl TypeConstraints {
    pub fn check(self, ty: &Type, attr: String) -> Result<(), Error> {
        match (self, ty) {
            (TypeConstraints::Verbatim, Type::Verbatim(_))
            | (TypeConstraints::Array, Type::Array(_))
            | (TypeConstraints::BareFn, Type::BareFn(_))
            | (TypeConstraints::Group, Type::Group(_))
            | (TypeConstraints::ImplTrait, Type::ImplTrait(_))
            | (TypeConstraints::Infer, Type::Infer(_))
            | (TypeConstraints::Macro, Type::Macro(_))
            | (TypeConstraints::Never, Type::Never(_))
            | (TypeConstraints::Paren, Type::Paren(_))
            | (TypeConstraints::Path, Type::Path(_))
            | (TypeConstraints::Ptr, Type::Ptr(_))
            | (TypeConstraints::Reference, Type::Reference(_))
            | (TypeConstraints::Slice, Type::Slice(_))
            | (TypeConstraints::TraitObject, Type::TraitObject(_))
            | (TypeConstraints::Tuple, Type::Tuple(_)) => Ok(()),
            _ => Err(Error::AttrValueTypeMimatch(attr)),
        }
    }
}

impl Attr {
    pub fn with(name: &str, attrs: &Vec<Attribute>) -> Result<Self, Error> {
        SingularAttr::with(name, attrs)
            .map(|singular| Attr::Singular(singular))
            .or_else(|_| ParametrizedAttr::with(name, attrs).map(|param| Attr::Parametrized(param)))
    }

    /// Constructor parsing [`Attribute`] value and returning either
    /// [`SingularAttr`] or [`ParametrizedAttr`] packed in form of [`Attr`]
    /// enum.
    ///
    /// If the attribute does not match either of forms, a [`Error`] is
    /// returned. Currently, only single type of error may occur in practice:
    /// - [`Error::ArgNameMustBeIdent`], which happens if the attribute name is
    ///   not an [`Ident`] but is a complex path value
    pub fn from_attribute(attr: &Attribute) -> Result<Self, Error> {
        SingularAttr::from_attribute(attr)
            .map(|singular| Attr::Singular(singular))
            .or_else(|_| {
                ParametrizedAttr::from_attribute(attr).map(|param| Attr::Parametrized(param))
            })
    }

    #[inline]
    pub fn try_singular(self) -> Result<SingularAttr, Error> {
        match self {
            Attr::Singular(attr) => Ok(attr),
            Attr::Parametrized(attr) => Err(Error::SingularAttrRequired(attr.name)),
        }
    }

    #[inline]
    pub fn try_parametrized(self) -> Result<ParametrizedAttr, Error> {
        match self {
            Attr::Singular(attr) => Err(Error::ParametrizedAttrRequired(attr.name)),
            Attr::Parametrized(attr) => Ok(attr),
        }
    }

    #[inline]
    pub fn name(&self) -> String {
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
}

impl SingularAttr {
    #[inline]
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }

    pub fn with(name: impl ToString, attrs: &Vec<Attribute>) -> Result<Self, Error> {
        let name = name.to_string();
        let mut filtered_attrs = attrs.iter().filter(|attr| attr.path.is_ident(&name));
        let res = if let Some(attr) = filtered_attrs.next() {
            SingularAttr::from_attribute(attr)
        } else {
            return Err(Error::SingularAttrRequired(name));
        };
        if filtered_attrs.count() > 0 {
            return Err(Error::SingularAttrRequired(name));
        }
        res
    }

    #[inline]
    pub fn with_named_literal(name: impl ToString, lit: Lit) -> Self {
        Self {
            name: name.to_string(),
            value: Some(ArgValue::Literal(lit)),
        }
    }

    pub fn from_attribute(attr: &Attribute) -> Result<Self, Error> {
        let ident = attr
            .path
            .get_ident()
            .ok_or(Error::ArgNameMustBeIdent)?
            .to_string();
        match attr.parse_meta()? {
            // `#[attr::path]` - unreachable: filtered in the code above
            Meta::Path(_) => unreachable!(),
            // `#[ident = lit]`
            Meta::NameValue(MetaNameValue { lit, .. }) => {
                Ok(SingularAttr::with_named_literal(ident, lit))
            }
            // `#[ident(...)]`
            Meta::List(_) => Err(Error::SingularAttrRequired(ident)),
        }
    }

    #[inline]
    pub fn value(&self) -> Result<ArgValue, Error> {
        self.value
            .as_ref()
            .cloned()
            .ok_or(Error::ArgValueRequired(self.name.clone()))
    }

    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        self.value()?.literal_value()
    }

    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        self.value()?.type_value()
    }

    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        if self.name != other.name {
            return Err(Error::NamesDontMatch(self.name.clone(), other.name.clone()));
        }
        match (&self.value, &other.value) {
            (_, None) => {}
            (None, Some(_)) => self.value = other.value,
            (Some(_), Some(_)) => return Err(Error::MultipleSingularValues(self.name.clone())),
        }
        Ok(())
    }

    #[inline]
    pub fn merged(mut self, other: Self) -> Result<Self, Error> {
        self.merge(other)?;
        Ok(self)
    }

    #[inline]
    pub fn enrich(&mut self, attr: &Attribute) -> Result<(), Error> {
        self.merge(SingularAttr::from_attribute(attr)?)
    }

    #[inline]
    pub fn enriched(mut self, attr: &Attribute) -> Result<Self, Error> {
        self.enrich(attr)?;
        Ok(self)
    }

    pub fn check(&mut self, req: ValueReq) -> Result<(), Error> {
        req.occurrences.check(&mut self.value, self.name.clone())?;
        if let Some(ref value) = self.value {
            req.constraints.check(value, self.name.clone())?;
        }
        Ok(())
    }

    #[inline]
    pub fn checked(mut self, req: ValueReq) -> Result<Self, Error> {
        self.check(req)?;
        Ok(self)
    }
}

impl ParametrizedAttr {
    #[inline]
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            args: Default::default(),
            paths: vec![],
            integers: vec![],
            literal: None,
        }
    }

    pub fn with(name: impl ToString + AsRef<str>, attrs: &Vec<Attribute>) -> Result<Self, Error> {
        let mut me = ParametrizedAttr::new(name.to_string());
        for attr in attrs.iter().filter(|attr| attr.path.is_ident(&name)) {
            match attr.parse_meta()? {
                // `#[ident(...)]`
                Meta::List(MetaList { nested, .. }) => {
                    for meta in nested {
                        me.fuse(meta)?;
                    }
                }
                _ => return Err(Error::ParametrizedAttrRequired(name.to_string())),
            }
        }
        Ok(me)
    }

    pub fn from_attribute(attr: &Attribute) -> Result<Self, Error> {
        let name = attr
            .path
            .get_ident()
            .ok_or(Error::ArgNameMustBeIdent)?
            .to_string();
        match attr.parse_meta()? {
            // `#[ident(...)]`
            Meta::List(MetaList { nested, .. }) => nested
                .into_iter()
                .fold(Ok(ParametrizedAttr::new(name)), |res, nested| {
                    res.and_then(|attr| attr.fused(nested))
                }),
            _ => Err(Error::ParametrizedAttrRequired(name)),
        }
    }

    pub fn arg_literal_value(&self, name: &str) -> Result<Lit, Error> {
        self.args
            .get(name)
            .ok_or(Error::NamedArgRequired(name.to_owned()))?
            .literal_value()
    }

    pub fn has_verbatim(&self, verbatim: &str) -> bool {
        self.paths
            .iter()
            .find(|path| path.is_ident(verbatim))
            .is_some()
    }

    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        if self.name != other.name {
            return Err(Error::NamesDontMatch(self.name.clone(), other.name.clone()));
        }
        self.args.extend(other.args);
        self.paths.extend(other.paths);
        self.integers.extend(other.integers);
        let span = self.literal.span();
        match (&mut self.literal, &other.literal) {
            (_, None) => {}
            (None, Some(_)) => self.literal = other.literal,
            (Some(Lit::Str(str1)), Some(Lit::Str(str2))) => {
                let mut joined = str1.value();
                joined.push_str(&str2.value());
                *str1 = LitStr::new(&joined, span);
            }
            (Some(_), Some(_)) => return Err(Error::MultipleLiteralValues(self.name.clone())),
        }
        Ok(())
    }

    #[inline]
    pub fn merged(mut self, other: Self) -> Result<Self, Error> {
        self.merge(other)?;
        Ok(self)
    }

    #[inline]
    pub fn enrich(&mut self, attr: &Attribute) -> Result<(), Error> {
        self.merge(ParametrizedAttr::from_attribute(attr)?)
    }

    #[inline]
    pub fn enriched(mut self, attr: &Attribute) -> Result<Self, Error> {
        self.enrich(attr)?;
        Ok(self)
    }

    #[inline]
    pub fn fuse(&mut self, nested: NestedMeta) -> Result<(), Error> {
        match nested {
            // `#[ident("literal", ...)]`
            NestedMeta::Lit(Lit::Str(str2)) => {
                let span = self.literal.span();
                match self.literal {
                    None => self.literal = Some(Lit::Str(str2)),
                    Some(Lit::Str(ref mut str1)) => {
                        let mut joined = str1.value();
                        joined.push_str(&str2.value());
                        *str1 = LitStr::new(&joined, span);
                    }
                    Some(_) => return Err(Error::MultipleLiteralValues(self.name.clone())),
                }
            }

            // `#[ident(3, ...)]`
            NestedMeta::Lit(Lit::Int(litint)) => self.integers.push(litint),

            // `#[ident(other_literal, ...)]`
            NestedMeta::Lit(lit) => self
                .literal
                .as_mut()
                .map(|l| *l = lit)
                .ok_or(Error::MultipleLiteralValues(self.name.clone()))?,

            // `#[ident(arg::path)]`
            NestedMeta::Meta(Meta::Path(path)) => self.paths.push(path),

            // `#[ident(name = value, ...)]`
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) => {
                let id = path
                    .clone()
                    .get_ident()
                    .ok_or(Error::ArgNameMustBeIdent)?
                    .to_string();
                if self
                    .args
                    .insert(id.clone(), ArgValue::Literal(lit))
                    .is_some()
                {
                    return Err(Error::ArgNameMustBeUnique(id));
                }
            }

            // `#[ident(arg(...), ...)]`
            NestedMeta::Meta(Meta::List(_)) => {
                return Err(Error::NestedListsNotSupported(self.name.clone()))
            }
        }
        Ok(())
    }

    #[inline]
    pub fn fused(mut self, nested: NestedMeta) -> Result<Self, Error> {
        self.fuse(nested)?;
        Ok(self)
    }

    pub fn check(&mut self, req: AttrReq) -> Result<(), Error> {
        match (req.paths, self.paths.len()) {
            (ListOccurrences::Deny, x) if x > 0 => {
                return Err(Error::AttrMustNotHavePaths(self.name.clone()))
            }
            (ListOccurrences::OneOrMore, 0) => {
                return Err(Error::AttrMustHavePath(self.name.clone()))
            }
            (ListOccurrences::Default(path), 0) => self.paths.push(path),
            _ => {}
        }

        match (req.integers, self.integers.len()) {
            (ListOccurrences::Deny, x) if x > 0 => {
                return Err(Error::AttrMustNotHavePaths(self.name.clone()))
            }
            (ListOccurrences::OneOrMore, 0) => {
                return Err(Error::AttrMustHavePath(self.name.clone()))
            }
            (ListOccurrences::Default(int), 0) => self.integers.push(int),
            _ => {}
        }

        match (req.literal.1, self.literal.as_ref()) {
            (ValueOccurrences::Prohibited, Some(_)) => {
                return Err(Error::AttrMustNotHaveLiteral(self.name.clone()))
            }
            (ValueOccurrences::Required, None) => {
                return Err(Error::AttrMustHaveLiteral(self.name.clone()))
            }
            (ValueOccurrences::Default(lit), None) => {
                self.literal = Some(lit.literal_value().expect(&format!(
                    "Argument default value for {} attribute must be a literal",
                    self.name
                )));
            }
            _ => {}
        }

        if let Some(ref mut lit) = self.literal {
            req.literal.0.check(&lit, self.name.clone())?;
        }

        for (name, req) in req.args {
            match (self.args.get(&name), req.occurrences) {
                (None, ValueOccurrences::Default(default)) => {
                    self.args.insert(name, default);
                }
                (None, occ) => {
                    occ.check(&mut None, self.name.clone())?;
                }
                (Some(val), occ) => {
                    occ.check(&mut Some(val.clone()), self.name.clone())?;
                    req.constraints.check(val, self.name.clone())?;
                }
            }
        }

        Ok(())
    }

    #[inline]
    pub fn checked(mut self, req: AttrReq) -> Result<Self, Error> {
        self.check(req)?;
        Ok(self)
    }
}

#[doc(hidden)]
pub trait ExtractAttr {
    #[doc(hidden)]
    fn singular_attr(
        self,
        name: impl ToString + AsRef<str>,
        req: ValueReq,
    ) -> Result<Option<SingularAttr>, Error>;

    #[doc(hidden)]
    fn parametrized_attr(
        self,
        name: impl ToString + AsRef<str>,
        req: AttrReq,
    ) -> Result<Option<ParametrizedAttr>, Error>;
}

impl<'a, T> ExtractAttr for T
where
    T: IntoIterator<Item = &'a Attribute>,
{
    /// Returns a [`SingularAttr`] which structure must fulfill the provided
    /// requirements - or fails with a [`Error`] otherwise. For more information
    /// check [`ValueReq`] requirements info.
    fn singular_attr(
        self,
        name: impl ToString + AsRef<str>,
        req: ValueReq,
    ) -> Result<Option<SingularAttr>, Error> {
        let mut attr = SingularAttr::new(name.to_string());

        let filtered = self
            .into_iter()
            .filter(|attr| attr.path.is_ident(&name))
            .collect::<Vec<_>>();

        if filtered.is_empty() {
            return Ok(None);
        }

        for entries in filtered {
            attr.enrich(entries)?;
        }

        Some(attr.checked(req)).transpose()
    }

    /// Returns a [`ParametrizedAttr`] which structure must fulfill the provided
    /// requirements - or fails with a [`Error`] otherwise. For more information
    /// check [`AttrReq`] requirements info.
    fn parametrized_attr(
        self,
        name: impl ToString + AsRef<str>,
        req: AttrReq,
    ) -> Result<Option<ParametrizedAttr>, Error> {
        let mut attr = ParametrizedAttr::new(name.to_string());

        let filtered = self
            .into_iter()
            .filter(|attr| attr.path.is_ident(&name))
            .collect::<Vec<_>>();

        if filtered.is_empty() {
            return Ok(None);
        }

        for entries in filtered {
            attr.enrich(entries)?;
        }

        Some(attr.checked(req)).transpose()
    }
}
