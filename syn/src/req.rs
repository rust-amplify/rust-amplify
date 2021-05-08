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

use std::collections::{HashMap};
use std::convert::TryInto;
use syn::{Path, LitChar, LitInt, LitFloat};
use quote::ToTokens;

use crate::{Error, ValueClass, ArgValue};

/// Structure requirements for parametrized attribute
#[derive(Clone)]
pub struct AttrReq {
    /// Specifies all named arguments and which requirements they must meet
    pub arg_req: HashMap<String, ArgValueReq>,

    /// Specifies whether path arguments are allowed and with which
    /// requirements.
    pub path_req: ListReq<Path>,

    /// Whether integer literals are allowed as an attribute argument and, if
    /// yes, with which requirements
    pub char_req: ListReq<LitChar>,

    /// Whether integer literals are allowed as an attribute argument and, if
    /// yes, with which requirements
    pub integer_req: ListReq<LitInt>,

    /// Whether integer literals are allowed as an attribute argument and, if
    /// yes, with which requirements
    pub float_req: ListReq<LitFloat>,

    /// Whether string literal is allowed as an attribute argument and, if
    /// yes, with which requirements
    pub string_req: ValueReq,

    /// Whether byte string literal is allowed as an attribute argument and, if
    /// yes, with which requirements
    pub bytes_req: ValueReq,

    /// Whether boolean literal is allowed as an attribute argument and, if
    /// yes, with which requirements
    pub bool_req: ValueReq,
}

impl AttrReq {
    /// Constructor creating [`AttrReq`] accepting only name-value arguments
    /// with the provided parameters
    pub fn with(args: HashMap<&str, ArgValueReq>) -> AttrReq {
        let args = args
            .into_iter()
            .map(|(name, req)| (name.to_owned(), req))
            .collect();

        AttrReq {
            arg_req: args,
            path_req: ListReq::Deny,
            char_req: ListReq::Deny,
            integer_req: ListReq::Deny,
            float_req: ListReq::Deny,
            string_req: ValueReq::Prohibited,
            bytes_req: ValueReq::Prohibited,
            bool_req: ValueReq::Prohibited,
        }
    }
}

/// Requirements for attribute or named argument value presence
#[derive(Clone)]
pub enum ArgValueReq {
    /// Argument must hold a value with the provided class
    Required {
        /// Default value
        default: Option<ArgValue>,
        /// Type of the value literal
        class: ValueClass,
    },

    /// Argument or an attribute may or may not hold a value
    Optional(ValueClass),

    /// Argument or an attribute must not hold a value
    Prohibited,
}

impl ArgValueReq {
    /// Constructs argument requirements object with default value
    pub fn with_default(default: impl Into<ArgValue>) -> ArgValueReq {
        let value = default.into();
        ArgValueReq::Required {
            class: value
                .value_class()
                .expect("Default argument value must not be `ArgValue::None`"),
            default: Some(value),
        }
    }

    /// Construct [`ArgValueReq::Required`] variant with no default value
    pub fn required(class: ValueClass) -> ArgValueReq {
        ArgValueReq::Required {
            default: None,
            class,
        }
    }

    /// Returns value class requirements, if any
    pub fn value_class(&self) -> Option<ValueClass> {
        match self {
            ArgValueReq::Required { class, .. } | ArgValueReq::Optional(class) => Some(*class),
            ArgValueReq::Prohibited => None,
        }
    }

    /// Returns default argument value. If not default is provided within the
    /// requirement, returns [`ArgValue::None`] (since this is *de facto*
    /// default value for any argument).
    pub fn default_value(&self) -> ArgValue {
        match self {
            ArgValueReq::Required {
                default: Some(d), ..
            } => d.clone(),
            _ => ArgValue::None,
        }
    }

    /// Determines whether argument is required to have a value
    pub fn is_required(&self) -> bool {
        match self {
            ArgValueReq::Required { default: None, .. } => true,
            _ => false,
        }
    }

    /// Checks the argument against current requirements, generating [`Error`]
    /// if the requirements are not met.
    pub fn check(
        &self,
        value: &mut ArgValue,
        attr: impl ToString,
        arg: impl ToString,
    ) -> Result<(), Error> {
        let value = match (value, self) {
            (val, ArgValueReq::Required { default: None, .. }) if val.is_none() => {
                return Err(Error::ArgValueRequired {
                    attr: attr.to_string(),
                    arg: arg.to_string(),
                })
            }

            (val, ArgValueReq::Prohibited) if val.is_some() => {
                return Err(Error::ArgMustNotHaveValue {
                    attr: attr.to_string(),
                    arg: arg.to_string(),
                })
            }

            (
                val,
                ArgValueReq::Required {
                    default: Some(d), ..
                },
            ) if val.value_class() != d.value_class() && val.value_class().is_some() => {
                panic!(
                    "Default value class {:?} does not match argument value class {:?} for attribute {}, argument {}",
                    d.value_class(),
                    val.value_class(),
                    attr.to_string(),
                    arg.to_string()
                );
            }

            (
                val,
                ArgValueReq::Required {
                    default: Some(d), ..
                },
            ) if val.is_none() => {
                *val = d.clone();
                val
            }

            // We are fine here
            (val, _) => val,
        };

        if let Some(value_class) = self.value_class() {
            value_class.check(value, attr, arg)?;
        }

        Ok(())
    }
}

/// Requirements for attribute or named argument value presence for a values
/// with known class. If the value class is not known, use [`ArgValueReq`]
/// instead.
#[derive(Clone)]
pub enum ValueReq {
    /// Argument or an attribute must hold a value
    Required,

    /// Argument or an attribute must hold a value; if the value is not present
    /// it will be substituted for the default value provided as the inner field
    Default(ArgValue),

    /// Argument or an attribute may or may not hold a value
    Optional,

    /// Argument or an attribute must not hold a value
    Prohibited,
}

impl ValueReq {
    /// Detects if the presence of the value is required
    #[inline]
    pub fn is_required(&self) -> bool {
        match self {
            ValueReq::Required => true,
            _ => false,
        }
    }

    /// Checks the value against current requirements, generating [`Error`] if
    /// the requirements are not met.
    pub fn check<T>(
        &self,
        value: &mut T,
        attr: impl ToString,
        arg: impl ToString,
    ) -> Result<(), Error>
    where
        T: Clone + Into<ArgValue>,
        ArgValue: TryInto<T>,
        Error: From<<ArgValue as TryInto<T>>::Error>,
    {
        let attr = attr.to_string();
        match (self, value) {
            (ValueReq::Required, v) if v.clone().into().is_none() => Err(Error::ArgValueRequired {
                attr,
                arg: arg.to_string(),
            }),
            (ValueReq::Prohibited, v) if v.clone().into().is_some() => {
                Err(Error::ArgMustNotHaveValue {
                    attr,
                    arg: arg.to_string(),
                })
            }
            (ValueReq::Default(ref val), v) if v.clone().into().is_none() => {
                *v = val.clone().try_into()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

/// Requirements for list elements. For instance, used in [`AttrReq`] for
/// providing [`ParametrizedAttr`] fields requirements.
#[derive(Clone)]
pub enum ListReq<T>
where
    T: Clone,
{
    /// Only a single value allowed and it must be present
    Single {
        /// Restricts set of possible values to the given whitelist
        ///
        /// NB: If whitelist does not contain values from the `default` field,
        /// they are still accepted as valid, i.e. "automatically whitelisted"
        whitelist: Option<Vec<T>>,

        /// Default value assigned as a signe list item if no values are
        /// provided
        ///
        /// NB: If whitelist does not contain values from the `default` field,
        /// they are still accepted as valid, i.e. "automatically whitelisted"
        default: Option<T>,
    },

    /// Any number of any elements may be present
    Many {
        /// Restricts set of possible values to the given whitelist
        whitelist: Option<Vec<T>>,

        /// Require that at least one value is present
        required: bool,

        /// Restricts the maximum number of items
        max_no: Option<u8>,
    },

    /// Any number of any elements may not be present; if none of the elements
    /// is present the list will use default vec of the values
    Predefined {
        /// Restricts set of possible values to the given whitelist.
        ///
        /// NB: If whitelist does not contain values from the `default` field,
        /// they are still accepted as valid, i.e. "automatically whitelisted"
        whitelist: Option<Vec<T>>,

        /// Default set of values for the list used if no values are provided
        ///
        /// NB: If whitelist does not contain values from the `default` field,
        /// they are still accepted as valid, i.e. "automatically whitelisted"
        default: Vec<T>,
    },

    /// Element must not be present
    Deny,
}

impl<T> ListReq<T>
where
    T: Clone + ToTokens,
{
    /// Checks the value against the list requirements, generating [`Error`] if
    /// the requirements are not met.
    pub fn check(
        &self,
        value: &mut Vec<T>,
        attr: impl ToString,
        arg: impl ToString,
    ) -> Result<(), Error> {
        match (self, value.len()) {
            // Checking are we allowed to have a value
            (ListReq::Deny, x) if x > 0 => {
                return Err(Error::ArgTypeProhibited {
                    attr: attr.to_string(),
                    arg: arg.to_string(),
                })
            }

            // Checking are we required to have a value while no value is available
            (ListReq::Many { required: true, .. }, 0)
            | (ListReq::Single { default: None, .. }, 0) => {
                return Err(Error::ArgRequired {
                    attr: attr.to_string(),
                    arg: arg.to_string(),
                })
            }

            // Checking not to the exceed maximally allowed number of values
            (
                ListReq::Many {
                    max_no: Some(max_no),
                    ..
                },
                no,
            ) if no > *max_no as usize => {
                return Err(Error::ArgNumberExceedsMax {
                    attr: attr.to_string(),
                    type_name: arg.to_string(),
                    no,
                    max_no: *max_no,
                })
            }

            // Checking that arguments are matching whitelist
            (
                ListReq::Many {
                    whitelist: Some(whitelist),
                    ..
                },
                len,
            )
            | (
                ListReq::Predefined {
                    whitelist: Some(whitelist),
                    ..
                },
                len,
            )
            | (
                ListReq::Single {
                    whitelist: Some(whitelist),
                    ..
                },
                len,
            ) if len > 0 => {
                for item in value {
                    if !whitelist.iter().any(|i| {
                        i.to_token_stream().to_string() == item.to_token_stream().to_string()
                    }) {
                        return Err(Error::AttributeUnknownArgument {
                            attr: attr.to_string(),
                            arg: arg.to_string(),
                        });
                    }
                }
            }

            // Defaulting if no value is provided
            (
                ListReq::Single {
                    default: Some(d), ..
                },
                0,
            ) => value.push(d.clone()),
            (ListReq::Predefined { default, .. }, 0) => *value = default.clone(),

            // Otherwise we are good
            _ => {}
        }
        Ok(())
    }
}
