// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
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

/// Trait for splittable streams and other types, which can be separated into
/// some two types ([`Bipolar::Left`], [`Bipolar::Right`]), like a reader and
/// writer streams.
pub trait Bipolar {
    /// First separable type (like reader)
    type Left;
    /// Second separable type (like writer)
    type Right;

    /// Reconstruct the type from the halves
    fn join(left: Self::Left, right: Self::Right) -> Self;

    /// Split the type into two
    fn split(self) -> (Self::Left, Self::Right);
}
