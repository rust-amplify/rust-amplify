// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Martin Habovstiak <martin.habovstiak@gmail.com>
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

//! This is a trick for rust compiler helping to distinguish types implementing
//! mutually-exclusive traits (required until negative trait impls will be
//! there) Implemented after concept by Martin Habovštiak
//! <martin.habovstiak@gmail.com>

use ::core::marker::PhantomData;

/// Helper type allowing implementation of trait object for generic types
/// multiple times. In practice this type is never used
pub struct Holder<T, S>(T, PhantomData<S>);
impl<T, S> Holder<T, S> {
    #[allow(missing_docs)]
    #[inline]
    pub fn new(val: T) -> Self {
        Self(val, PhantomData::<S>::default())
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn as_inner(&self) -> &T {
        &self.0
    }
}
