// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[macro_use]
mod wrapper;
mod as_any;
mod dumb;
mod join_split;
#[cfg(all(feature = "c_raw", not(target_arch = "wasm32")))]
mod raw_str;

pub use as_any::AsAny;
pub use join_split::JoinSplit;
pub use wrapper::{Wrapper, WrapperMut};
pub use dumb::Dumb;
#[cfg(all(feature = "c_raw", not(target_arch = "wasm32")))]
pub use raw_str::{TryFromRawStr, TryAsStr, TryIntoRawStr, TryIntoString};
