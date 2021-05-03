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

impl LiteralClass {
    /// Checks the literal against current requirements, generating [`Error`] if
    /// the requirements are not met.
    pub fn check(self, lit: &Lit, attr: impl ToString, arg: impl ToString) -> Result<(), Error> {
        match (self, lit) {
            (LiteralClass::BoolLiteral, Lit::Bool(_))
            | (LiteralClass::ByteLiteral, Lit::Byte(_))
            | (LiteralClass::ByteStrLiteral, Lit::ByteStr(_))
            | (LiteralClass::CharLiteral, Lit::Char(_))
            | (LiteralClass::FloatLiteral, Lit::Float(_))
            | (LiteralClass::IntLiteral, Lit::Int(_))
            | (LiteralClass::StringLiteral, Lit::Str(_))
            | (LiteralClass::Verbatim, Lit::Verbatim(_)) => Ok(()),
            _ => Err(Error::ArgValueTypeMismatch {
                attr: attr.to_string(),
                arg: arg.to_string(),
            }),
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

impl TypeClass {
    /// Checks the [`Type`] against current requirements, generating [`Error`]
    /// if the requirements are not met.
    pub fn check(self, ty: &Type, attr: impl ToString, arg: impl ToString) -> Result<(), Error> {
        match (self, ty) {
            (TypeClass::Verbatim, Type::Verbatim(_))
            | (TypeClass::Array, Type::Array(_))
            | (TypeClass::BareFn, Type::BareFn(_))
            | (TypeClass::Group, Type::Group(_))
            | (TypeClass::ImplTrait, Type::ImplTrait(_))
            | (TypeClass::Infer, Type::Infer(_))
            | (TypeClass::Macro, Type::Macro(_))
            | (TypeClass::Never, Type::Never(_))
            | (TypeClass::Paren, Type::Paren(_))
            | (TypeClass::Path, Type::Path(_))
            | (TypeClass::Ptr, Type::Ptr(_))
            | (TypeClass::Reference, Type::Reference(_))
            | (TypeClass::Slice, Type::Slice(_))
            | (TypeClass::TraitObject, Type::TraitObject(_))
            | (TypeClass::Tuple, Type::Tuple(_)) => Ok(()),
            _ => Err(Error::ArgValueTypeMismatch {
                attr: attr.to_string(),
                arg: arg.to_string(),
            }),
        }
    }
}
