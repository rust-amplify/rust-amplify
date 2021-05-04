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
use std::convert::TryInto;
use syn::{
    Type, Ident, Path, Lit, LitStr, LitByteStr, LitBool, LitChar, LitInt, LitFloat, TypePath,
    PathSegment,
};
use syn::parse::{Parse, ParseBuffer, Parser};
use proc_macro2::{TokenStream, Span};
use quote::{ToTokens};

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

impl Debug for ArgValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ArgValue::Literal(lit) => write!(f, "ArgValue::Literal({})", lit.to_token_stream()),
            ArgValue::Type(ty) => write!(f, "ArgValue::Type({})", ty.to_token_stream()),
            ArgValue::None => f.write_str("ArgValue::None"),
        }
    }
}

impl From<&str> for ArgValue {
    fn from(val: &str) -> Self {
        ArgValue::Literal(Lit::Str(LitStr::new(val, Span::call_site())))
    }
}

impl From<String> for ArgValue {
    fn from(val: String) -> Self {
        ArgValue::Literal(Lit::Str(LitStr::new(&val, Span::call_site())))
    }
}

impl From<&[u8]> for ArgValue {
    fn from(val: &[u8]) -> Self {
        ArgValue::Literal(Lit::ByteStr(LitByteStr::new(val, Span::call_site())))
    }
}

impl From<Vec<u8>> for ArgValue {
    fn from(val: Vec<u8>) -> Self {
        ArgValue::Literal(Lit::ByteStr(LitByteStr::new(&val, Span::call_site())))
    }
}

impl From<char> for ArgValue {
    fn from(val: char) -> Self {
        ArgValue::Literal(Lit::Char(LitChar::new(val, Span::call_site())))
    }
}

impl From<usize> for ArgValue {
    fn from(val: usize) -> Self {
        ArgValue::Literal(Lit::Int(LitInt::new(&val.to_string(), Span::call_site())))
    }
}

impl From<isize> for ArgValue {
    fn from(val: isize) -> Self {
        ArgValue::Literal(Lit::Int(LitInt::new(&val.to_string(), Span::call_site())))
    }
}

impl From<Ident> for ArgValue {
    fn from(ident: Ident) -> Self {
        Path::from(PathSegment::parse.parse(quote! { #ident }.into()).unwrap()).into()
    }
}

impl From<Path> for ArgValue {
    fn from(path: Path) -> Self {
        ArgValue::Type(Type::Path(TypePath { qself: None, path }))
    }
}

impl From<f64> for ArgValue {
    fn from(val: f64) -> Self {
        ArgValue::Literal(Lit::Float(LitFloat::new(
            &val.to_string(),
            Span::call_site(),
        )))
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

impl From<Option<LitChar>> for ArgValue {
    fn from(val: Option<LitChar>) -> Self {
        match val {
            Some(val) => ArgValue::Literal(Lit::Char(val)),
            None => ArgValue::None,
        }
    }
}

impl From<Option<LitInt>> for ArgValue {
    fn from(val: Option<LitInt>) -> Self {
        match val {
            Some(val) => ArgValue::Literal(Lit::Int(val)),
            None => ArgValue::None,
        }
    }
}

impl From<Option<LitFloat>> for ArgValue {
    fn from(val: Option<LitFloat>) -> Self {
        match val {
            Some(val) => ArgValue::Literal(Lit::Float(val)),
            None => ArgValue::None,
        }
    }
}

impl TryInto<String> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Str(s)) => Ok(s.value()),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Vec<u8>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::ByteStr(s)) => Ok(s.value()),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<bool> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Bool(b)) => Ok(b.value),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<char> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Char(c)) => Ok(c.value()),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<LitStr> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<LitStr, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Str(s)) => Ok(s),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<LitByteStr> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<LitByteStr, Self::Error> {
        match self {
            ArgValue::Literal(Lit::ByteStr(s)) => Ok(s),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<LitBool> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<LitBool, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Bool(s)) => Ok(s),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<LitChar> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<LitChar, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Char(c)) => Ok(c),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<LitInt> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<LitInt, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Int(i)) => Ok(i),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<LitFloat> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<LitFloat, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Float(f)) => Ok(f),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Ident> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Ident, Self::Error> {
        match self {
            ArgValue::Type(Type::Path(ty)) => {
                if let Some(ident) = ty.path.get_ident() {
                    Ok(ident.clone())
                } else {
                    Err(Error::ArgValueMustBeType)
                }
            }
            _ => Err(Error::ArgValueMustBeType),
        }
    }
}

impl TryInto<Path> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Path, Self::Error> {
        match self {
            ArgValue::Type(Type::Path(ty)) => Ok(ty.path),
            _ => Err(Error::ArgValueMustBeType),
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

impl TryInto<Option<LitChar>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<LitChar>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Char(c)) => Ok(Some(c)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Option<LitInt>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<LitInt>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Int(i)) => Ok(Some(i)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Option<LitFloat>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<LitFloat>, Self::Error> {
        match self {
            ArgValue::Literal(Lit::Float(f)) => Ok(Some(f)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeLiteral),
        }
    }
}

impl TryInto<Option<Ident>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<Ident>, Self::Error> {
        match self {
            ArgValue::Type(Type::Path(ty)) => {
                if let Some(ident) = ty.path.get_ident() {
                    Ok(Some(ident.clone()))
                } else {
                    Err(Error::ArgValueMustBeType)
                }
            }
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeType),
        }
    }
}

impl TryInto<Option<Path>> for ArgValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<Path>, Self::Error> {
        match self {
            ArgValue::Type(Type::Path(ty)) => Ok(Some(ty.path)),
            ArgValue::None => Ok(None),
            _ => Err(Error::ArgValueMustBeType),
        }
    }
}

impl Parse for ArgValue {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        if let Ok(lit) = Lit::parse(input) {
            Ok(ArgValue::Literal(lit))
        } else if let Ok(ty) = Type::parse(input) {
            Ok(ArgValue::Type(ty))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Attribute argument value must be a rust literal or a type",
            ))
        }
    }
}

impl ToTokens for ArgValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ArgValue::Literal(lit) => lit.to_tokens(tokens),
            ArgValue::Type(ty) => ty.to_tokens(tokens),
            ArgValue::None => quote! { ! }.to_tokens(tokens),
        }
    }
}

impl ArgValue {
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
