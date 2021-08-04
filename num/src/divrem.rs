// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
// Refactored & fixed in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore
// Contributed in 2021 by
//     Jose Diego Robles Pardo <jd.robles@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

pub trait DivRem {
    fn div_rem(self, other: Self) -> (Self, Self)
    where
        Self: Sized;
    fn div_rem_checked(self, other: Self) -> Option<(Self, Self)>
    where
        Self: Sized;
}