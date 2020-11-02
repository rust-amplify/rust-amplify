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

/// Used as an alternative to default for test and prototyping purposes, when a
/// type can't have a default value, but you need to generate some dumb data.
pub trait DumbDefault
where
    Self: Sized,
{
    /// Returns an object initialized with dumb data
    fn dumb_default() -> Self;
}
