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

use std::fmt::{Display, Formatter, self};
use std::convert::Infallible;
use proc_macro2::Span;

/// Errors representing inconsistency in proc macro attribute structure
#[derive(Clone, Debug)]
pub enum Error {
    /// Parse error from a `syn` crate
    Parse(syn::Error),

    /// Names of two merged attributes must match each other
    NamesDontMatch(String, String),

    /// Singular argument (of form `#[attr = ...]`) has multiple occurrences
    /// each assigned value. This is meaningless.
    MultipleSingularValues(String),

    /// Multiple literal non-string values are given for a parametrized
    /// attribute in form of `#[attr(literal1, literal2)]`. This is
    /// meaningless.
    MultipleLiteralValues(String),

    /// Attribute contains unsupported literal argument
    UnsupportedLiteral(String),

    /// Attribute must be in a singular form (`#[attr]` or `#[attr = ...]`)
    SingularAttrRequired(String),

    /// Attribute must be in a parametrized form (`#[attr(...)]`)
    ParametrizedAttrRequired(String),

    /// Attribute has an unknown argument
    AttributeUnknownArgument {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Attribute is not allowed to have argument of type `arg`
    ArgTypeProhibited {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Number of `arg` arguments in attribute `attr` exceeds maximum
    ArgNumberExceedsMax {
        /// Attribute name
        attr: String,
        /// Argument type name
        type_name: String,
        /// Number of arguments
        no: usize,
        /// Maximum allowed number of arguments
        max_no: u8,
    },

    /// Attribute argument `arg` must not have a value
    ArgMustNotHaveValue {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Attribute must has an `arg` argument
    ArgRequired {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Attribute or attribute argument name (in form of `#[attr(arg = ...)]`)
    /// must be an identifier (like `arg`) and not a path (like `std::io`)
    ArgNameMustBeIdent,

    /// The same argument name is used multiple times within parametrized
    /// attribute (like in `#[attr(name1 = value1, name1 = value2)]`)
    ArgNameMustBeUnique {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Attribute or attribute argument must has a value:
    /// `#[attr(arg = value)]`
    ArgValueRequired {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Attribute value type mismatch
    ArgValueTypeMismatch {
        /// Attribute name
        attr: String,
        /// Argument name
        arg: String,
    },

    /// Parametrized attribute argument must have a literal value (string,
    /// integer etc): `#[attr(arg = "value")]` or `#[arg = 4]`
    ArgValueMustBeLiteral,

    /// Parametrized attribute argument must be a valid type name:
    /// `#[attr(arg = u8)]` or `#[arg = String]`
    ArgValueMustBeType,

    /// Parametrized attribute (in form of `#[attr(...)]`) does not
    /// have a single value
    ParametrizedAttrHasNoValue(String),

    /// Lists nested within attribute arguments, like `#[attr(arg(...))]`
    /// are not supported
    NestedListsNotSupported(String),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::Parse(err)
    }
}

impl From<Error> for syn::Error {
    fn from(err: Error) -> Self {
        syn::Error::new(Span::call_site(), err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "attribute parse error: {}", err),
            Error::NamesDontMatch(name1, name2) => write!(
                f,
                "Names of two merged attributes (`{}` and `{}`) must match",
                name1,
                name2
            ),
            Error::MultipleSingularValues(name) => write!(
                f,
                "Multiple values assigned to `{}` attribute",
                name
            ),
            Error::MultipleLiteralValues(name) => write!(
                f,
                "Multiple literal values provided for `{}` attribute",
                name
            ),
            Error::SingularAttrRequired(name) => write!(
                f,
                "Attribute `{}` must be in a singular form (`#[attr]` or `#[attr = ...]`)",
                name
            ),
            Error::ParametrizedAttrRequired(name) => write!(
                f,
                "Attribute `{}` must be in a parametrized form (`#[attr(...)]`)",
                name
            ),
            Error::ArgMustNotHaveValue { attr, arg } => write!(
                f,
                "Argument {arg} in `{attr}` attribute must not have a value",
                attr = attr, arg = arg
            ),
            Error::ArgTypeProhibited { attr, arg: type_name } => write!(
                f,
                "Attribute `{}` prohibits arguments of type `{}`",
                attr, type_name
            ),
            Error::ArgRequired { attr, arg } => write!(
                f,
                "Attribute `{}` requires argument `{}` to be explicitly specified",
                attr, arg,
            ),
            Error::ArgNameMustBeUnique { attr, arg } => write!(
                f,
                "Argument names must be unique, while attribute `{}` contains multiple arguments with name`{}`",
                attr, arg,
            ),
            Error::ArgNameMustBeIdent => write!(
                f,
                "Attribute arguments must be identifiers, not paths",
            ),
            Error::ArgValueRequired { attr, arg } => write!(
                f,
                "Attribute `{}` requires value for argument `{}`",
                attr, arg
            ),
            Error::ArgValueMustBeLiteral => f.write_str(
                "Attribute argument value must be a literal (string, int etc)",
            ),
            Error::ArgValueMustBeType => {
                f.write_str("Attribute value for must be a valid type name")
            }
            Error::ParametrizedAttrHasNoValue(name) => {
                write!(
                    f,
                    "Attribute `{name}` must be in a `#[{name} = ...]` form",
                    name = name
                )
            }
            Error::NestedListsNotSupported(name) => write!(
                f,
                "Attribute `{name}` must be in `{name} = ...` form and a nested list",
                name = name,
            ),
            Error::UnsupportedLiteral(attr) => write!(
                f,
                "Attribute `{}` has an unsupported type of literal as one of its arguments",
                attr
            ),
            Error::AttributeUnknownArgument { attr, arg } => write!(
                f,
                "Attribute `{}` has an unknown argument `{}`",
                attr, arg
            ),
            Error::ArgNumberExceedsMax { attr, type_name, no, max_no } => write!(
                f,
                "Attribute `{}` has excessive number of arguments of type `{}` ({} while only {} are allowed)",
                attr, type_name, no, max_no
            ),
            Error::ArgValueTypeMismatch { attr, arg } => write!(
                f,
                "Type mismatch in attribute `{}` argument `{}`",
                attr, arg
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(err) => Some(err),
            Error::NamesDontMatch(_, _)
            | Error::MultipleSingularValues(_)
            | Error::MultipleLiteralValues(_)
            | Error::SingularAttrRequired(_)
            | Error::ArgMustNotHaveValue { .. }
            | Error::ArgTypeProhibited { .. }
            | Error::ArgRequired { .. }
            | Error::ParametrizedAttrRequired(_)
            | Error::ArgNameMustBeIdent
            | Error::ArgNameMustBeUnique { .. }
            | Error::ArgValueRequired { .. }
            | Error::ArgValueMustBeLiteral
            | Error::ArgValueMustBeType
            | Error::ParametrizedAttrHasNoValue(_)
            | Error::UnsupportedLiteral(_)
            | Error::AttributeUnknownArgument { .. }
            | Error::ArgNumberExceedsMax { .. }
            | Error::ArgValueTypeMismatch { .. }
            | Error::NestedListsNotSupported(_) => None,
        }
    }
}
