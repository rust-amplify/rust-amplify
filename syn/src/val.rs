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

use std::convert::TryInto;
use syn::{Type, Lit, LitStr, LitByteStr, LitBool};
use proc_macro2::{TokenStream, Span};

use crate::{Error, ValueClass};

/// Value for attribute or attribute argument, i.e. for `#[attr = value]` and
/// `#[attr(arg = value)]` this is the `value` part of the attribute. Can be
/// either a single literal or a single valid rust type name
#[derive(Clone)]
pub enum ArgValue {
    /// Attribute value represented by a literal
    Literal(Lit),

    /// Attribute value represented by a type name
    Type(Type),

    /// No value is given
    None,
}

impl From<&str> for ArgValue {
    fn from(val: &str) -> Self {
        ArgValue::Literal(Lit::Str(LitStr::new(val, Span::call_site())))
    }
}

impl From<Option<LitStr>> for ArgValue {
    fn from(val: Option<LitStr>) -> Self {
        match val {
            Some(val) => ArgValue::Literal(Lit::Str(val)),
            None => ArgValue::None,
        }
    }
}

impl From<Option<LitByteStr>> for ArgValue {
    fn from(val: Option<LitByteStr>) -> Self {
        match val {
            Some(val) => ArgValue::Literal(Lit::ByteStr(val)),
            None => ArgValue::None,
        }
    }
}

impl From<Option<LitBool>> for ArgValue {
    fn from(val: Option<LitBool>) -> Self {
        match val {
            Some(val) => ArgValue::Literal(Lit::Bool(val)),
            None => ArgValue::None,
        }
    }
}

impl TryInto<Option<LitStr>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<LitStr>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Str(s)) => Ok(Some(s)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Option<LitByteStr>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<LitByteStr>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::ByteStr(s)) => Ok(Some(s)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Option<LitBool>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<LitBool>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Bool(b)) => Ok(Some(b)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl ArgValue {
    /// Helper method converting [`ArgValue`] into a [`TokenStream`].
    ///
    /// We can't `impl ToTokens for ArgValue`, since `ToTokens` trait is a
    /// private inside `syn` crate, so we can't support direct use of
    /// [`ArgValue`] inside `quote!` and `quote_spanned!` macros. Instead, use
    /// this method to acquire [`TokenStream`] variable and use it in quotations
    #[inline]
    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            ArgValue::Literal(lit) => quote! { #lit },
            ArgValue::Type(ty) => quote! { #ty },
            ArgValue::None => quote! {},
        }
    }

    /// Returns literal value for [`ArgValue::Literal`] variant or fails with
    /// [`Error::ArgValueMustBeLiteral`] otherwise
    #[inline]
    pub fn literal_value(&self) -> Result<Lit, Error> {
        match self {
            ArgValue::Literal(lit) => Ok(lit.clone()),
            ArgValue::Type(_) | ArgValue::None => Err(Error::ArgValueMustBeLiteral),
        }
    }

    /// Returns type value for [`ArgValue::Type`] variant or fails with
    /// [`Error::ArgValueMustBeType`] otherwise
    #[inline]
    pub fn type_value(&self) -> Result<Type, Error> {
        match self {
            ArgValue::Literal(_) | ArgValue::None => Err(Error::ArgValueMustBeType),
            ArgValue::Type(ty) => Ok(ty.clone()),
        }
    }

    /// Tests whether the self is set to [`ArgValue::None`]
    #[inline]
    pub fn is_none(&self) -> bool {
        match self {
            ArgValue::None => true,
            _ => false,
        }
    }

    /// Tests whether the self is not set to [`ArgValue::None`]
    #[inline]
    pub fn is_some(&self) -> bool {
        match self {
            ArgValue::None => false,
            _ => true,
        }
    }

    /// Returns [`ValueClass`] for the current value, if any
    #[inline]
    pub fn value_class(&self) -> Option<ValueClass> {
        match self {
            ArgValue::Literal(lit) => Some(ValueClass::from(lit)),
            ArgValue::Type(ty) => Some(ValueClass::from(ty)),
            ArgValue::None => None,
        }
    }
}
