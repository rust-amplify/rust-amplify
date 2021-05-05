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

use std::fmt::{Debug, Formatter, self};
use std::collections::{HashMap, HashSet};
use syn::{
    Type, Path, Attribute, Meta, MetaNameValue, Lit, LitInt, LitStr, LitByteStr, LitFloat, LitChar,
    LitBool,
};
use syn::parse_quote::ParseQuote;
use syn::parse::Parser;

use crate::{Error, ArgValue, ArgValueReq, AttrReq, MetaArg, MetaArgNameValue, MetaArgList};

/// Internal structure representation of a proc macro attribute collected
/// instances having some specific name (accessible via [`Attr::name()`]).
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct SingularAttr {
    /// Optional attribute argument path part; for instance in
    /// `#[my(name = value)]` or in `#[name = value]` this is a `name` part
    pub name: String,

    /// Attribute argument value part; for instance in `#[name = value]` this is
    /// the `value` part
    pub value: ArgValue,
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
    /// or `#[attr(ident = TypeName)]` mapped to their name identifiers.
    pub args: HashMap<String, ArgValue>,

    /// All attribute arguments that are paths or identifiers without any
    /// specific value, like `#[attr(std::io::Error, crate, super::SomeType)]`.
    ///
    /// NB: All named arguments without the value assigned are getting into this
    /// field by default, even if they do not represent any known rust path or
    /// type name, like `#[attr(some_id, other)]`. However, with
    /// [`ParametrizedAttr::check`] and [`ParametrizedAttr::checked`] those
    /// values matching ones specified in [`AttrReq::args`] with values set to
    /// [`ValueOccurrences::Default`] are moved into [`ParametrizedAttr::args`].
    pub paths: Vec<Path>,

    /// Unnamed string literal found within attribute arguments.
    ///
    /// If multiple string literals are present they are concatenated into a
    /// single value, like it is done by the rust compiler for
    /// `#[doc = "..."]` attributes
    pub string: Option<LitStr>,

    /// Unnamed byte string literal found within attribute arguments.
    ///
    /// If multiple byte string literals are present they are concatenated into
    /// a single value, like it is done by the rust compiler for
    /// `#[doc = "..."]` attributes
    pub bytes: Option<LitByteStr>,

    /// Unnamed char literals found within attribute arguments
    pub chars: Vec<LitChar>,

    /// Unnamed integer literals found within attribute arguments
    pub integers: Vec<LitInt>,

    /// Unnamed float literals found within attribute arguments
    pub floats: Vec<LitFloat>,

    /// Unnamed bool literal found within attribute arguments.
    ///
    /// If multiple bool literals are present this will generate an error.
    pub bool: Option<LitBool>,
}

impl Attr {
    /// Constructs [`Attr`] from a vector of all syn-parsed attributes,
    /// selecting attributes matching the provided name.
    pub fn with(name: impl ToString + AsRef<str>, attrs: &Vec<Attribute>) -> Result<Self, Error> {
        SingularAttr::with(name.to_string(), attrs)
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

    /// Returns inner value \in form of [`SingularAttr`] for [`Attr::Singular`]
    /// variant, or fails with [`Error::SingularAttrRequired`] otherwise
    #[inline]
    pub fn try_singular(self) -> Result<SingularAttr, Error> {
        match self {
            Attr::Singular(attr) => Ok(attr),
            Attr::Parametrized(attr) => Err(Error::SingularAttrRequired(attr.name)),
        }
    }

    /// Returns inner value \in form of [`ParametrizedAttr`] for
    /// [`Attr::Parametrized`] variant, or fails with
    /// [`Error::ParametrizedAttrRequired`] otherwise
    #[inline]
    pub fn try_parametrized(self) -> Result<ParametrizedAttr, Error> {
        match self {
            Attr::Singular(attr) => Err(Error::ParametrizedAttrRequired(attr.name)),
            Attr::Parametrized(attr) => Ok(attr),
        }
    }

    /// Returns string reference to the argument name
    #[inline]
    pub fn name(&self) -> &str {
        match self {
            Attr::Singular(attr) => &attr.name,
            Attr::Parametrized(attr) => &attr.name,
        }
    }

    /// Returns [`ArgValue`] for the [`Attr::Singular`] variant or fails with
    /// [`Error::ParametrizedAttrHasNoValue`]
    #[inline]
    pub fn arg_value(&self) -> Result<ArgValue, Error> {
        match self {
            Attr::Singular(attr) => Ok(attr.value.clone()),
            Attr::Parametrized(attr) => Err(Error::ParametrizedAttrHasNoValue(attr.name.clone())),
        }
    }

    /// Returns literal value for the [`Attr::Singular`] variant or fails with
    /// [`Error::ParametrizedAttrHasNoValue`]. See [`ArgValue::literal_value`]
    /// for more details.
    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        self.arg_value()?.literal_value()
    }

    /// Returns type value for the [`Attr::Singular`] variant or fails with
    /// [`Error::ParametrizedAttrHasNoValue`]. See [`ArgValue::literal_value`]
    /// for more details.
    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        self.arg_value()?.type_value()
    }
}

impl SingularAttr {
    /// Constructs named [`SingularAttr`] without value
    #[inline]
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: ArgValue::None,
        }
    }

    /// Constructs [`SingularAttr`] from a vector of all syn-parsed attributes,
    /// selecting single attribute matching the provided name. If there are
    /// multiple instances of the same attribute, fails with
    /// [`Error::SingularAttrRequired`]
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

    /// Constructs named [`SingularAttr`] setting its value to the provided
    /// literal
    #[inline]
    pub fn with_literal(name: impl ToString, lit: Lit) -> Self {
        Self {
            name: name.to_string(),
            value: ArgValue::Literal(lit),
        }
    }

    /// Constructs named [`SingularAttr`] setting its value to the provided
    /// rust type value
    #[inline]
    pub fn with_type(name: impl ToString, ty: Type) -> Self {
        Self {
            name: name.to_string(),
            value: ArgValue::Type(ty),
        }
    }

    /// Constructs [`SingularAttr`] from a given [`syn::Attribute`] by parsing
    /// its data. Accepts only attributes having form `#[attr(name = value)]`
    /// and errors for other attribute types with [`Error::ArgNameMustBeIdent`]
    /// and [`Error::SingularAttrRequired`]
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
                Ok(SingularAttr::with_literal(ident, lit))
            }
            // `#[ident(...)]`
            Meta::List(_) => Err(Error::SingularAttrRequired(ident)),
        }
    }

    /// Returns literal value, if any, or fails with
    /// [`Error::ArgValueRequired`]. See [`ArgValue::literal_value`] for the
    /// details.
    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        self.value.literal_value()
    }

    /// Returns type value, if any, or fails with [`Error::ArgValueRequired`].
    /// See [`ArgValue::literal_value`] for the details.
    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        self.value.type_value()
    }

    /// Merges data from the `other` into the self.
    ///
    /// # Errors
    ///
    /// - Fails with [`Error::NamesDontMatch`] if the names of the self and the
    ///   `other` do not match
    /// - Fails with [`Error::MultipleSingularValues`] if both self and the
    ///   `other` has a named argument with the same name but different values.
    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        if self.name != other.name {
            return Err(Error::NamesDontMatch(self.name.clone(), other.name.clone()));
        }
        match (&self.value, &other.value) {
            (_, ArgValue::None) => {}
            (ArgValue::None, _) => self.value = other.value,
            (_, _) => return Err(Error::MultipleSingularValues(self.name.clone())),
        }
        Ok(())
    }

    /// Does merging as in [`SingularAttr::merge`], but unlike it consumes
    /// the self and returns a merged structure in case of the successful
    /// operation. Useful in operation chains.
    #[inline]
    pub fn merged(mut self, other: Self) -> Result<Self, Error> {
        self.merge(other)?;
        Ok(self)
    }

    /// Enriches current attribute data by adding information from the provided
    /// [`syn::Attribute`].
    ///
    /// # Errors
    ///
    /// - Fails with [`Error::NamesDontMatch`] if the names of the self and the
    ///   provided attribute do not match
    /// - Fails with [`Error::MultipleSingularValues`] if both self and the
    ///   provided attribute has a named argument with the same name but
    ///   different values.
    #[inline]
    pub fn enrich(&mut self, attr: &Attribute) -> Result<(), Error> {
        self.merge(SingularAttr::from_attribute(attr)?)
    }

    /// Performs enrich operation as in [`SingularAttr::enrich`], but unlike it
    /// consumes the self and returns an enriched structure in case of the
    /// successful operation. Useful in operation chains.
    #[inline]
    pub fn enriched(mut self, attr: &Attribute) -> Result<Self, Error> {
        self.enrich(attr)?;
        Ok(self)
    }

    /// Checks that the structure meets provided value requirements (see
    /// [`ValueReq`]), generating [`Error`] if the requirements are not met.
    pub fn check(&mut self, req: ArgValueReq) -> Result<(), Error> {
        req.check(&mut self.value, &self.name, &self.name)?;
        Ok(())
    }

    /// Performs check as in [`SingularAttr::check`], but unlike it consumes the
    /// self and returns a itself in case of the successful operation.
    /// Useful in operation chains.
    #[inline]
    pub fn checked(mut self, req: ArgValueReq) -> Result<Self, Error> {
        self.check(req)?;
        Ok(self)
    }
}

impl ParametrizedAttr {
    /// Constructs named [`SingularAttr`] with empty internal data
    #[inline]
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            args: Default::default(),
            paths: vec![],
            string: None,
            bytes: None,
            chars: vec![],
            integers: vec![],
            floats: vec![],
            bool: None,
        }
    }

    /// Constructs [`ParametrizedAttr`] from a vector of all syn-parsed
    /// attributes, selecting attributes matching the provided name.
    pub fn with(name: impl ToString + AsRef<str>, attrs: &Vec<Attribute>) -> Result<Self, Error> {
        let mut me = ParametrizedAttr::new(name.to_string());
        for attr in attrs.iter().filter(|attr| attr.path.is_ident(&name)) {
            me.fuse(attr)?;
        }
        Ok(me)
    }

    /// Constructs [`ParametrizedAttr`] from a given [`syn::Attribute`]
    pub fn from_attribute(attr: &Attribute) -> Result<Self, Error> {
        let name = attr
            .path
            .get_ident()
            .ok_or(Error::ArgNameMustBeIdent)?
            .to_string();
        ParametrizedAttr::new(name).fused(attr)
    }

    /// Returns literal value for a given argument with name `name`, if it is
    /// defined, or fails with [`Error::ArgValueRequired`]. See
    /// [`ArgValue::literal_value`] for the details.
    pub fn arg_literal_value(&self, name: &str) -> Result<Lit, Error> {
        self.args
            .get(name)
            .ok_or(Error::ArgRequired {
                attr: self.name.clone(),
                arg: name.to_owned(),
            })?
            .literal_value()
    }

    /// Checks if the attribute has a verbatim argument matching the provided
    /// `verbatim` string.
    ///
    /// Verbatim arguments are arguments in form of `#[attr(verbatim1,
    /// verbatim2]`, i.e. path arguments containing single path segment and no
    /// value or nested arguments.
    pub fn has_verbatim(&self, verbatim: &str) -> bool {
        self.paths
            .iter()
            .find(|path| path.is_ident(verbatim))
            .is_some()
    }

    /// Returns set of verbatim attribute arguments.
    ///
    /// Verbatim arguments are arguments in form of `#[attr(verbatim1,
    /// verbatim2]`, i.e. path arguments containing single path segment and no
    /// value or nested arguments.
    pub fn verbatim(&self) -> HashSet<String> {
        self.paths
            .iter()
            .filter_map(Path::get_ident)
            .map(|ident| ident.to_string())
            .collect()
    }

    /// Merges data from the `other` into the self.
    ///
    /// # Errors
    ///
    /// - Fails with [`Error::NamesDontMatch`] if the names of the self and the
    ///   `other` do not match
    /// - Fails with [`Error::MultipleLiteralValues`] if both self and the
    ///   `other` has a literals which values are not equal.
    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        if self.name != other.name {
            return Err(Error::NamesDontMatch(self.name.clone(), other.name.clone()));
        }

        self.args.extend(other.args);
        self.paths.extend(other.paths);

        self.integers.extend(other.integers);
        self.floats.extend(other.floats);
        self.chars.extend(other.chars);

        match (&mut self.string, &other.string) {
            (_, None) => {}
            (None, Some(_)) => self.string = other.string.clone(),
            (Some(str1), Some(str2)) => {
                *str1 = LitStr::new(&format!("{} {}", str1.value(), str2.value()), str1.span());
            }
        }
        match (&mut self.bytes, &other.bytes) {
            (_, None) => {}
            (None, Some(_)) => self.bytes = other.bytes.clone(),
            (Some(bytes1), Some(bytes2)) => {
                let mut joined = bytes1.value();
                joined.extend(bytes2.value());
                *bytes1 = LitByteStr::new(&joined, bytes1.span());
            }
        }
        match (&mut self.bool, &other.bool) {
            (_, None) => {}
            (None, Some(_)) => self.bool = other.bool.clone(),
            (Some(_), Some(_)) => return Err(Error::MultipleLiteralValues(self.name.clone())),
        }

        Ok(())
    }

    /// Does merging as in [`ParametrizedAttr::merge`], but unlike it consumes
    /// the self and returns a merged structure in case of the successful
    /// operation. Useful in operation chains.
    #[inline]
    pub fn merged(mut self, other: Self) -> Result<Self, Error> {
        self.merge(other)?;
        Ok(self)
    }

    /// Enriches current attribute data by adding information from the provided
    /// [`syn::Attribute`].
    ///
    /// # Errors
    ///
    /// - Fails with [`Error::NamesDontMatch`] if the names of the self and the
    ///   provided attribute do not match
    /// - Fails with [`Error::MultipleLiteralValues`] if both self and the
    ///   provided attribute has a literals which values are not equal.
    #[inline]
    pub fn enrich(&mut self, attr: &Attribute) -> Result<(), Error> {
        self.merge(ParametrizedAttr::from_attribute(attr)?)
    }

    /// Performs enrich operation as in [`ParametrizedAttr::enrich`], but unlike
    /// it consumes the self and returns an enriched structure in case of
    /// the successful operation. Useful in operation chains.
    #[inline]
    pub fn enriched(mut self, attr: &Attribute) -> Result<Self, Error> {
        self.enrich(attr)?;
        Ok(self)
    }

    /// Fuses data from a nested attribute arguments (see [`Attribute`]) into
    /// the attribute parameters.
    ///
    /// The operation is similar to the [`ParametrizedAttr::enrich`] with the
    /// only difference that enrichment operation takes the whole attribute, and
    /// fusion takes a nested meta data.
    #[inline]
    pub fn fuse(&mut self, attr: &Attribute) -> Result<(), Error> {
        let args = MetaArgList::parse.parse(attr.tokens.clone().into())?;
        for arg in args.list {
            match arg {
                // `#[ident("literal", ...)]`
                MetaArg::Literal(Lit::Str(s)) => {
                    let span = s.span();
                    match self.string {
                        None => self.string = Some(s),
                        Some(ref mut str1) => {
                            let mut joined = str1.value();
                            joined.push_str(&s.value());
                            *str1 = LitStr::new(&joined, span);
                        }
                    }
                }

                // `#[ident(b"literal", ...)]`
                MetaArg::Literal(Lit::ByteStr(s)) => {
                    let span = s.span();
                    match self.bytes {
                        None => self.bytes = Some(s),
                        Some(ref mut str1) => {
                            let mut joined = str1.value();
                            joined.extend(&s.value());
                            *str1 = LitByteStr::new(&joined, span);
                        }
                    }
                }

                // `#[ident(3, ...)]`
                MetaArg::Literal(Lit::Int(lit)) => self.integers.push(lit),

                // `#[ident(2.3, ...)]`
                MetaArg::Literal(Lit::Float(lit)) => self.floats.push(lit),

                // `#[ident('a', ...)]`
                MetaArg::Literal(Lit::Char(lit)) => self.chars.push(lit),

                // `#[ident(true, ...)]`
                MetaArg::Literal(Lit::Bool(_)) if self.bool.is_some() => {
                    return Err(Error::MultipleLiteralValues(self.name.clone()))
                }
                MetaArg::Literal(Lit::Bool(lit)) if self.bool.is_none() => {
                    self.bool = Some(lit.clone())
                }

                // `#[ident(true, ...)]`
                MetaArg::Literal(_) => return Err(Error::UnsupportedLiteral(self.name.clone())),

                // `#[ident(arg::path)]`
                MetaArg::Path(path) => self.paths.push(path),

                // `#[ident(name = value, ...)]`
                MetaArg::NameValue(MetaArgNameValue { name, value, .. }) => {
                    let id = name.to_string();
                    if self.args.insert(id.clone(), value).is_some() {
                        return Err(Error::ArgNameMustBeUnique {
                            attr: self.name.clone(),
                            arg: id,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Performs enrich operation as in [`ParametrizedAttr::fuse`], but unlike
    /// it consumes the self and returns an enriched structure in case of
    /// the successful operation. Useful in operation chains.
    #[inline]
    pub fn fused(mut self, attr: &Attribute) -> Result<Self, Error> {
        self.fuse(attr)?;
        Ok(self)
    }

    /// Checks that the structure meets provided value requirements (see
    /// [`AttrReq`]), generating [`Error`] if the requirements are not met.
    ///
    /// The procedure modifies the [`ParametrizedAttr`] data in the following
    /// ways:
    /// 1. First, it fills in [`ParametrizedAttr::paths`],
    ///    [`ParametrizedAttr::integers`] and [`ParametrizedAttr::literal`] with
    ///    default values    from [`AttrReq::paths`], [`AttrReq::integers`] and
    ///    [`AttrReq::literal`] (correspondingly).
    /// 2. [`ParametrizedAttr::paths`] values matching ones specified in
    ///    [`AttrReq::args`] with values set to [`ValueOccurrences::Default`]
    ///    are moved into [`ParametrizedAttr::args`] field.
    pub fn check(&mut self, req: AttrReq) -> Result<(), Error> {
        for (name, req) in &req.arg_req {
            if let Some(pos) = self.paths.iter().position(|path| path.is_ident(name)) {
                self.paths.remove(pos);
                self.args.entry(name.clone()).or_insert(req.default_value());
            }

            if !self.args.contains_key(name) && req.is_required() {
                return Err(Error::ArgRequired {
                    attr: self.name.clone(),
                    arg: name.clone(),
                });
            }
        }

        for (name, value) in &mut self.args {
            let req = if let Some(req) = req.arg_req.get(name) {
                req
            } else {
                return Err(Error::AttributeUnknownArgument {
                    attr: self.name.clone(),
                    arg: name.clone(),
                });
            };

            req.check(value, &self.name, name)?;
        }

        req.path_req.check(&mut self.paths, &self.name, "path")?;

        req.integer_req
            .check(&mut self.integers, &self.name, "integer literal")?;
        req.float_req
            .check(&mut self.floats, &self.name, "float literal")?;
        req.char_req
            .check(&mut self.chars, &self.name, "char literal")?;

        req.string_req
            .check(&mut self.string, &self.name, "string literal")?;
        req.bool_req
            .check(&mut self.bool, &self.name, "bool literal")?;
        req.bytes_req
            .check(&mut self.bytes, &self.name, "byte string literal")?;

        Ok(())
    }

    /// Performs check as in [`ParametrizedAttr::check`], but unlike it
    /// consumes the self and returns a itself in case of the successful
    /// operation. Useful in operation chains.
    #[inline]
    pub fn checked(mut self, req: AttrReq) -> Result<Self, Error> {
        self.check(req)?;
        Ok(self)
    }
}

// This trait should not be implemented for the types outside of this crate
#[doc(hidden)]
pub trait ExtractAttr {
    #[doc(hidden)]
    fn singular_attr(
        self,
        name: impl ToString + AsRef<str>,
        req: ArgValueReq,
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
        req: ArgValueReq,
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

impl Debug for ParametrizedAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ParametrizedAttr({")?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        write!(f, "name: {:?}, ", self.name)?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        f.write_str("args: {")?;
        if !self.args.is_empty() {
            if f.alternate() {
                f.write_str("\n")?;
            }
            for (name, val) in &self.args {
                if f.alternate() {
                    f.write_str("\t\t")?;
                }
                write!(f, "{} => {:?}, ", name, val)?;
                if f.alternate() {
                    f.write_str("\n")?;
                }
            }
            if f.alternate() {
                f.write_str("\t")?;
            }
        }
        f.write_str("}, ")?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        f.write_str("paths: [")?;
        if !self.paths.is_empty() {
            if f.alternate() {
                f.write_str("\n")?;
            }
            for path in &self.paths {
                if f.alternate() {
                    f.write_str("\t\t")?;
                }
                write!(f, "{}, ", quote! { #path })?;
                if f.alternate() {
                    f.write_str("\n")?;
                }
            }
            if f.alternate() {
                f.write_str("\t")?;
            }
        }
        f.write_str("], ")?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        write!(
            f,
            "bool: {:?}, ",
            self.bool
                .as_ref()
                .map(|b| format!("Some({:?})", b.value))
                .unwrap_or("None".to_owned())
        )?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        write!(
            f,
            "string: {:?}, ",
            self.string
                .as_ref()
                .map(|s| format!("Some({:?})", s.value()))
                .unwrap_or("None".to_owned())
        )?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        write!(
            f,
            "bytes: {:?}, ",
            self.bytes
                .as_ref()
                .map(|s| format!("Some({:?})", s.value()))
                .unwrap_or("None".to_owned())
        )?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        f.write_str("chars: [")?;
        if !self.chars.is_empty() {
            if f.alternate() {
                f.write_str("\n")?;
            }
            for c in &self.chars {
                if f.alternate() {
                    f.write_str("\t\t")?;
                }
                write!(f, "{}, ", quote! { #c })?;
                if f.alternate() {
                    f.write_str("\n")?;
                }
            }
            if f.alternate() {
                f.write_str("\t")?;
            }
        }
        f.write_str("], ")?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        f.write_str("integers: [")?;
        if !self.integers.is_empty() {
            if f.alternate() {
                f.write_str("\n")?;
            }
            for c in &self.integers {
                if f.alternate() {
                    f.write_str("\t\t")?;
                }
                write!(f, "{}, ", quote! { #c })?;
                if f.alternate() {
                    f.write_str("\n")?;
                }
            }
            if f.alternate() {
                f.write_str("\t")?;
            }
        }
        f.write_str("], ")?;
        if f.alternate() {
            f.write_str("\n\t")?;
        }

        f.write_str("floats: [")?;
        if !self.floats.is_empty() {
            if f.alternate() {
                f.write_str("\n")?;
            }
            for c in &self.floats {
                if f.alternate() {
                    f.write_str("\t\t")?;
                }
                write!(f, "{}, ", quote! { #c })?;
                if f.alternate() {
                    f.write_str("\n")?;
                }
            }
            if f.alternate() {
                f.write_str("\t")?;
            }
        }
        f.write_str("], ")?;

        if f.alternate() {
            f.write_str("\n")?;
        }
        f.write_str("})")?;
        if f.alternate() {
            f.write_str("\n")?;
        }
        Ok(())
    }
}
