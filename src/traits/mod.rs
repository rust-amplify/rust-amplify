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
#[cfg(feature = "c_raw")]
mod raw_str;
#[cfg(feature = "serde")]
mod to_serde_string;

pub use as_any::AsAny;
pub use join_split::JoinSplit;
pub use wrapper::{Wrapper, WrapperMut};
pub use dumb::Dumb;
#[cfg(feature = "c_raw")]
pub use raw_str::{TryFromRawStr, TryAsStr, TryIntoRawStr, TryIntoString};
#[cfg(feature = "serde")]
pub use to_serde_string::{ToYamlString, ToJsonString, ToTomlString};
