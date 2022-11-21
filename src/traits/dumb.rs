// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2022 by
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

use crate::{Array, Wrapper};

/// Used as an alternative to default for test and prototyping purposes, when a
/// type can't have a default value, but you need to generate some dumb data.
pub trait Dumb
where
    Self: Sized,
{
    /// Returns an object initialized with dumb data
    fn dumb() -> Self;
}

impl Dumb for u8 {
    #[cfg(feature = "rand")]
    fn dumb() -> Self {
        use rand::RngCore;
        rand::thread_rng().next_u32().to_be_bytes()[0]
    }

    #[cfg(not(feature = "rand"))]
    fn dumb() -> Self {
        1
    }
}

impl<T, const LEN: usize> Dumb for Array<T, LEN>
where
    T: Dumb + Copy,
{
    fn dumb() -> Self {
        Self::from_inner([T::dumb(); LEN])
    }
}

// TODO: Implement for main primitive types
// TODO: Implement for main collection types
// TODO: Implement for types defined in this crate
