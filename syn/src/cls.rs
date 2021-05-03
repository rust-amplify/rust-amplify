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
use syn::{Type, Lit};

use crate::{Error, ArgValue};

/// Constrains for attribute value type
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ValueClass {
    /// The value must be a literal matching given literal constraints (see
    /// [`ConstrainedLit`])
    Literal(LiteralClass),

    /// The value must be of a native rust type matching given type constraints
    /// (see [`ConstrainedType`])
    Type(TypeClass),
}

impl From<Lit> for ValueClass {
    fn from(lit: Lit) -> Self {
        ValueClass::Literal(LiteralClass::from(lit))
    }
}

impl From<&Lit> for ValueClass {
    fn from(lit: &Lit) -> Self {
        ValueClass::Literal(LiteralClass::from(lit))
    }
}

impl From<Type> for ValueClass {
    fn from(ty: Type) -> Self {
        ValueClass::Type(TypeClass::from(ty))
    }
}

impl From<&Type> for ValueClass {
    fn from(ty: &Type) -> Self {
        ValueClass::Type(TypeClass::from(ty))
    }
}

impl ValueClass {
    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::Str)`
    pub fn str() -> ValueClass {
        ValueClass::Literal(LiteralClass::Str)
    }

    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::ByteStr)`
    pub fn byte_str() -> ValueClass {
        ValueClass::Literal(LiteralClass::ByteStr)
    }

    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::Byte)`
    pub fn byte() -> ValueClass {
        ValueClass::Literal(LiteralClass::Byte)
    }

    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::Int)`
    pub fn int() -> ValueClass {
        ValueClass::Literal(LiteralClass::Int)
    }

    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::Float)`
    pub fn float() -> ValueClass {
        ValueClass::Literal(LiteralClass::Float)
    }

    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::Char)`
    pub fn char() -> ValueClass {
        ValueClass::Literal(LiteralClass::Char)
    }

    /// Convenience constructor creating
    /// `ValueClass::Literal(LiteralClass::Bool)`
    pub fn bool() -> ValueClass {
        ValueClass::Literal(LiteralClass::Bool)
    }
}

impl ValueClass {
    /// Checks the value against value class requirements, generating [`Error`]
    /// if the requirements are not met.
    pub fn check(
        self,
        value: &ArgValue,
        attr: impl ToString,
        arg: impl ToString,
    ) -> Result<(), Error> {
        match (self, value) {
            (ValueClass::Literal(lit), ArgValue::Literal(ref value)) => lit.check(value, attr, arg),
            (ValueClass::Type(ty), ArgValue::Type(ref value)) => ty.check(value, attr, arg),
            _ => Err(Error::ArgValueTypeMismatch {
                attr: attr.to_string(),
                arg: arg.to_string(),
            }),
        }
    }
}

/// Constrains for literal value type
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum LiteralClass {
    /// Literal must be a string
    Str,

    /// Literal must be a byte string
    ByteStr,

    /// Literal must be a byte (in form of `b'f'`)
    Byte,

    /// Literal must be a character
    Char,

    /// Literal must be an integer
    Int,

    /// Literal must be a float
    Float,

    /// Literal must be a boolean
    Bool,

    /// Literal must be a verbatim form
    Verbatim,
}

impl From<Lit> for LiteralClass {
    #[inline]
    fn from(lit: Lit) -> Self {
        LiteralClass::from(&lit)
    }
}

impl From<&Lit> for LiteralClass {
    fn from(lit: &Lit) -> Self {
        match lit {
            Lit::Str(_) => LiteralClass::Str,
            Lit::ByteStr(_) => LiteralClass::ByteStr,
            Lit::Byte(_) => LiteralClass::Byte,
            Lit::Char(_) => LiteralClass::Char,
            Lit::Int(_) => LiteralClass::Int,
            Lit::Float(_) => LiteralClass::Float,
            Lit::Bool(_) => LiteralClass::Bool,
            Lit::Verbatim(_) => LiteralClass::Verbatim,
        }
    }
}

impl LiteralClass {
    /// Checks the literal against current requirements, generating [`Error`] if
    /// the requirements are not met.
    pub fn check(self, lit: &Lit, attr: impl ToString, arg: impl ToString) -> Result<(), Error> {
        if self != LiteralClass::from(lit) {
            Err(Error::ArgValueTypeMismatch {
                attr: attr.to_string(),
                arg: arg.to_string(),
            })
        } else {
            Ok(())
        }
    }
}

/// Constrains for the possible types that a Rust value could have.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeClass {
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

impl From<Type> for TypeClass {
    #[inline]
    fn from(ty: Type) -> Self {
        TypeClass::from(&ty)
    }
}

impl From<&Type> for TypeClass {
    fn from(ty: &Type) -> Self {
        match ty {
            Type::Array(_) => TypeClass::Array,
            Type::BareFn(_) => TypeClass::BareFn,
            Type::Group(_) => TypeClass::Group,
            Type::ImplTrait(_) => TypeClass::ImplTrait,
            Type::Infer(_) => TypeClass::Infer,
            Type::Macro(_) => TypeClass::Macro,
            Type::Never(_) => TypeClass::Never,
            Type::Paren(_) => TypeClass::Paren,
            Type::Path(_) => TypeClass::Path,
            Type::Ptr(_) => TypeClass::Ptr,
            Type::Reference(_) => TypeClass::Reference,
            Type::Slice(_) => TypeClass::Slice,
            Type::TraitObject(_) => TypeClass::TraitObject,
            Type::Tuple(_) => TypeClass::Tuple,
            Type::Verbatim(_) => TypeClass::Verbatim,
            _ => unreachable!(),
        }
    }
}

impl TypeClass {
    /// Checks the [`Type`] against current requirements, generating [`Error`]
    /// if the requirements are not met.
    pub fn check(self, ty: &Type, attr: impl ToString, arg: impl ToString) -> Result<(), Error> {
        if self != TypeClass::from(ty) {
            Err(Error::ArgValueTypeMismatch {
                attr: attr.to_string(),
                arg: arg.to_string(),
            })
        } else {
            Ok(())
        }
    }
}
