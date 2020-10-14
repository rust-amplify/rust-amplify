// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Martin Habovstiak <martin.habovstiak@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

/// Trait defining wrapped types ("newtypes" in rust terminology). Wrapped
/// types are used for allowing implemeting foreign traits to foreign types:
/// <https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html>
///
/// Trait defines convenient methods for accessing inner data, construct
/// and deconstruct newtype. It also serves as a marker trait for newtypes.
///
/// NB: Complete usage of this wrapper is possible only with nightly rust
/// compiler with `trivial_bounds` feature gate for the crate (see
/// example below) and `nightly` feature set. This will give you an automatic
/// implementation for all operation types, formatting etc.
///
/// ```
/// #![feature(trivial_bounds)]
/// # #[macro_use] extern crate amplify;
/// wrapper!(Bytes, Vec<u8>, doc = "Byte string");
///
/// let bytes = Bytes::from(vec![0, 1, 2, 3, 4, 5]);
/// assert_eq!(bytes[1], 1)
/// ```
pub trait Wrapper {
    /// Inner type wrapped by the current newtype
    type Inner: Clone;

    /// Instantiates wrapper type with the inner data
    fn from_inner(inner: Self::Inner) -> Self;

    /// Returns reference to the inner representation for the wrapper type
    fn as_inner(&self) -> &Self::Inner;

    /// Returns a mutable reference to the inner representation for the wrapper
    /// type
    fn as_inner_mut(&mut self) -> &mut Self::Inner;

    /// Clones inner data of the wrapped type and return them
    #[inline]
    fn to_inner(&self) -> Self::Inner {
        self.as_inner().clone()
    }

    /// Unwraps the wrapper returning the inner type
    fn into_inner(self) -> Self::Inner;
}

// TODO: Add generic support to the wrapper
// TODO: Convert to derive macro
/// Macro simplifying creation of new wrapped types. It automatically implements
/// [`Wrapper`] trait, adds default implementation for the following traits:
/// * [`AsRef`]
/// * [`AsMut`]
/// * [`Borrow`]
/// * [`BorrowMut`]
/// * [`Deref`]
/// * [`DerefMut`]
/// * [`From`][`<Wrapper>`]
/// * [`From`]`<Inner>`
///
/// Macro allows to add custom derives to the newtype using `derive` argument
#[macro_export]
macro_rules! wrapper {
    ($name:ident, $from:ty, $docs:meta) => {
        wrapper!($name, $from, $docs, derive=[]);
    };

    ($name:ident, $from:ty, $docs:meta, derive=[$( $derive:ident ),*]) => {
        #[$docs]
        $( #[derive($derive)] )*
        pub struct $name($from);

        impl_wrapper!($name, $from);
        impl_wrapped_traits!($name);

        impl_wrapper_from!($name, $from);
    };
}

/// Implements [`Wrapper`] trait for a given `$name` type
#[macro_export]
macro_rules! impl_wrapper {
    ($name:ident, $from:ty) => {
        impl_wrapper!($name, $from, 0);
    };

    ($name:ident, $from:ty, $field:tt) => {
        impl $crate::Wrapper for $name {
            type Inner = $from;

            #[inline]
            fn from_inner(inner: Self::Inner) -> Self {
                Self(inner)
            }

            #[inline]
            fn as_inner(&self) -> &Self::Inner {
                &self.$field
            }

            #[inline]
            fn as_inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.$field
            }

            #[inline]
            fn into_inner(self) -> Self::Inner {
                self.$field
            }
        }
    };
}

/// Implements [`From`][`Wrapper::Inner`]. Requires either:
/// - nightly toolchain, `trivial_bounds` feature gait and `nightly` feature
///   enabled
/// - or that the type implements `Default` trait
#[macro_export]
macro_rules! impl_wrapper_from {
    ($name:ident, $from:ty) => {
        impl ::core::convert::From<$from> for $name {
            #[inline]
            fn from(inner: <Self as $crate::Wrapper>::Inner) -> Self {
                Self(inner)
            }
        }

        #[allow(trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::convert::From<&$from> for $name
        where
            Self: ::core::clone::Clone,
        {
            #[inline]
            fn from(inner: &<Self as $crate::Wrapper>::Inner) -> Self {
                Self(inner.clone())
            }
        }
    };

    ($name:ident, $from:ty, ..Default::default()) => {
        #[allow(trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::convert::From<$from> for $name
        where
            Self: ::core::default::Default,
        {
            #[inline]
            fn from(inner: <Self as $crate::Wrapper>::Inner) -> Self {
                Self(inner, ..Self::default())
            }
        }

        #[allow(trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::convert::From<&$from> for $name
        where
            Self: ::core::clone::Clone + ::core::default::Default,
        {
            #[inline]
            fn from(inner: &<Self as $crate::Wrapper>::Inner) -> Self {
                Self(inner.clone(), ..Self::default())
            }
        }
    };

    ($name:ident, $from:ty, $field:ident) => {
        #[allow(trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::convert::From<$from> for $name
        where
            Self: ::core::default::Default,
        {
            #[inline]
            fn from(inner: <Self as $crate::Wrapper>::Inner) -> Self {
                Self {
                    $field: inner,
                    ..Self::default()
                }
            }
        }

        #[allow(trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::convert::From<&$from> for $name
        where
            Self: ::core::clone::Clone + ::core::default::Default,
        {
            #[inline]
            fn from(inner: &<Self as $crate::Wrapper>::Inner) -> Self {
                Self {
                    $field: inner.clone(),
                    ..Self::default()
                }
            }
        }
    };
}

/// Adds automatic implementation of traits implemented by the inner type to
/// the main wrapper type
#[macro_export]
macro_rules! impl_wrapped_traits {
    ($name:ident) => {
        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::clone::Clone for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::clone::Clone,
        {
            fn clone(&self) -> Self {
                use $crate::Wrapper;
                Self::from(self.as_inner().clone())
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::default::Default for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::default::Default,
        {
            fn default() -> Self {
                use $crate::Wrapper;
                Self::from(<Self as $crate::Wrapper>::Inner::default())
            }
        }

        #[allow(unused_imports)]
        impl ::core::convert::AsRef<<$name as $crate::Wrapper>::Inner> for $name {
            #[inline]
            fn as_ref(&self) -> &<Self as $crate::Wrapper>::Inner {
                use $crate::Wrapper;
                self.as_inner()
            }
        }

        #[allow(unused_imports)]
        impl ::core::convert::AsMut<<$name as $crate::Wrapper>::Inner> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut <Self as $crate::Wrapper>::Inner {
                use $crate::Wrapper;
                self.as_inner_mut()
            }
        }

        #[allow(unused_imports)]
        impl ::core::borrow::Borrow<<$name as $crate::Wrapper>::Inner> for $name {
            #[inline]
            fn borrow(&self) -> &<Self as $crate::Wrapper>::Inner {
                use $crate::Wrapper;
                self.as_inner()
            }
        }

        #[allow(unused_imports)]
        impl ::core::borrow::BorrowMut<<$name as $crate::Wrapper>::Inner> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut <Self as $crate::Wrapper>::Inner {
                use $crate::Wrapper;
                self.as_inner_mut()
            }
        }

        #[allow(unused_imports)]
        impl ::core::ops::Deref for $name {
            type Target = <Self as $crate::Wrapper>::Inner;
            #[inline]
            fn deref(&self) -> &Self::Target {
                use $crate::Wrapper;
                self.as_inner()
            }
        }

        #[allow(unused_imports)]
        impl ::core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                use $crate::Wrapper;
                self.as_inner_mut()
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::str::FromStr for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::str::FromStr,
        {
            type Err = <<Self as $crate::Wrapper>::Inner as ::std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $crate::Wrapper;
                Ok(Self::from_inner(
                    <Self as $crate::Wrapper>::Inner::from_str(s)?,
                ))
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::Display for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::Display,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::Debug for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::LowerHex for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::LowerHex,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::UpperHex for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::UpperHex,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::Octal for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::Octal,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::LowerExp for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::LowerExp,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::std::fmt::UpperExp for $name
        where
            <Self as $crate::Wrapper>::Inner: ::std::fmt::UpperExp,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $crate::Wrapper;
                self.as_inner().fmt(f)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl<T> ::core::ops::Index<T> for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::ops::Index<T>,
        {
            type Output = <<Self as $crate::Wrapper>::Inner as ::core::ops::Index<T>>::Output;

            fn index(&self, index: T) -> &Self::Output {
                use $crate::Wrapper;
                self.as_inner().index(index)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl<T> ::core::ops::IndexMut<T> for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::ops::IndexMut<T>,
        {
            fn index_mut(&mut self, index: T) -> &mut Self::Output {
                use $crate::Wrapper;
                self.as_inner_mut().index_mut(index)
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::Add for $name
        where
            <Self as $crate::Wrapper>::Inner:
                ::core::ops::Add<Output = <Self as $crate::Wrapper>::Inner>,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                use $crate::Wrapper;
                Self::from_inner(self.into_inner().add(rhs.into_inner()))
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::Sub for $name
        where
            <Self as $crate::Wrapper>::Inner:
                ::core::ops::Sub<Output = <Self as $crate::Wrapper>::Inner>,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                use $crate::Wrapper;
                Self::from_inner(self.into_inner().sub(rhs.into_inner()))
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::Mul for $name
        where
            <Self as $crate::Wrapper>::Inner:
                ::core::ops::Mul<Output = <Self as $crate::Wrapper>::Inner>,
        {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                use $crate::Wrapper;
                Self::from_inner(self.into_inner().mul(rhs.into_inner()))
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::Div for $name
        where
            <Self as $crate::Wrapper>::Inner:
                ::core::ops::Div<Output = <Self as $crate::Wrapper>::Inner>,
        {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                use $crate::Wrapper;
                Self::from_inner(self.into_inner().div(rhs.into_inner()))
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::AddAssign for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::ops::AddAssign,
        {
            fn add_assign(&mut self, rhs: Self) {
                use $crate::Wrapper;
                self.as_inner_mut().add_assign(rhs.into_inner())
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::SubAssign for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::ops::SubAssign,
        {
            fn sub_assign(&mut self, rhs: Self) {
                use $crate::Wrapper;
                self.as_inner_mut().sub_assign(rhs.into_inner())
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::MulAssign for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::ops::MulAssign,
        {
            fn mul_assign(&mut self, rhs: Self) {
                use $crate::Wrapper;
                self.as_inner_mut().mul_assign(rhs.into_inner())
            }
        }

        #[allow(unused_imports, trivial_bounds)]
        #[cfg(feature = "nightly")]
        impl ::core::ops::DivAssign for $name
        where
            <Self as $crate::Wrapper>::Inner: ::core::ops::DivAssign,
        {
            fn div_assign(&mut self, rhs: Self) {
                use $crate::Wrapper;
                self.as_inner_mut().div_assign(rhs.into_inner())
            }
        }
    };
}
