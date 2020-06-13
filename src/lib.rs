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

mod as_any;
#[macro_use]
mod convert;
#[cfg(feature = "serde")]
mod serde;
mod strategy;
mod wrapper;

pub use crate::as_any::AsAny;
#[cfg(feature = "serde")]
pub use crate::serde::CowHelper;
pub use crate::strategy::Holder;
pub use crate::wrapper::Wrapper;
