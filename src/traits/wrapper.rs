// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2022 by
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
/// The trait works well with `#[derive(Wrapper)]` from `amplify_derive` crate
pub trait Wrapper {
    /// Inner type wrapped by the current newtype
    type Inner;

    /// Instantiates wrapper type with the inner data
    fn from_inner(inner: Self::Inner) -> Self;

    /// Returns reference to the inner representation for the wrapper type
    fn as_inner(&self) -> &Self::Inner;

    /// Clones inner data of the wrapped type and return them
    #[inline]
    fn to_inner(&self) -> Self::Inner
    where
        Self::Inner: Clone,
    {
        self.as_inner().clone()
    }

    /// Unwraps the wrapper returning the inner type
    fn into_inner(self) -> Self::Inner;

    /// Copies the wrapped type
    fn copy(&self) -> Self
    where
        Self: Sized,
        Self::Inner: Copy,
    {
        Self::from_inner(*self.as_inner())
    }
}

/// Trait allowing mutable reference borrowing for the wrapped inner type.
pub trait WrapperMut: Wrapper {
    /// Returns a mutable reference to the inner representation for the wrapper
    /// type
    fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    struct TestWrapper(u8);

    impl Wrapper for TestWrapper {
        type Inner = u8;

        fn from_inner(inner: Self::Inner) -> Self {
            Self(inner)
        }

        fn as_inner(&self) -> &Self::Inner {
            &self.0
        }

        fn into_inner(self) -> Self::Inner {
            self.0
        }
    }

    impl WrapperMut for TestWrapper {
        fn as_inner_mut(&mut self) -> &mut Self::Inner {
            &mut self.0
        }
    }

    #[test]
    fn test_copy() {
        let item = TestWrapper::from_inner(5);
        let copy = item.copy();
        assert_eq!(item, copy);
        assert_eq!(copy.into_inner(), 5)
    }
}
