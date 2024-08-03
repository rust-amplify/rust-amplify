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

#[cfg(feature = "alloc")]
#[macro_use]
pub mod confinement;
mod array;
#[cfg(feature = "std")]
pub mod flags;

#[allow(deprecated)]
pub use array::{
    Array, Bytes, Bytes4, Bytes16, Bytes20, Bytes32, Bytes32StrRev, Bytes64, ByteArray, RawArray,
    FromSliceError,
};
#[cfg(feature = "std")]
pub use flags::{FlagRef, FlagNo, FlagVec};
