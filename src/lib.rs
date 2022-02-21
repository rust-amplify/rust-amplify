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

//! Amplifying Rust language capabilities: multiple generic trait
//! implementations, type wrappers, derive macros.
//!
//! Minimum supported rust compiler version (MSRV): 1.46 (stable channel)

#![recursion_limit = "256"]
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    dead_code,
    missing_docs,
    warnings
)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate core;

#[cfg(feature = "derive")]
#[macro_use]
extern crate amplify_derive;
#[cfg(feature = "derive")]
pub use amplify_derive::{Wrapper, Display, AsAny, From, Getters, Error};

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

extern crate amplify_num;
#[cfg(any(test, feature = "hex"))]
pub use num::hex;
#[cfg(feature = "stringly_conversions")]
pub extern crate stringly_conversions;
#[cfg(feature = "stringly_conversions")]
pub use stringly_conversions::*;
#[cfg(feature = "proc_attr")]
pub extern crate amplify_syn as proc_attr;
#[cfg(feature = "proc_attr")]
pub use proc_attr::ident;

#[macro_use]
mod macros;
#[macro_use]
mod wrapper;

mod as_any;
mod bipolar;
mod dumb_default;
#[cfg(all(feature = "std", feature = "derive"))]
mod io_error;
#[cfg(feature = "c_raw")]
mod raw;
mod slice32;
pub mod strategy;
#[cfg(feature = "serde")]
mod to_serde_string;

#[cfg(feature = "std")]
pub mod flags;

pub mod num {
    //! Custom-sized numeric types
    //!
    //! Implementation of a various integer types with custom bit dimension. These
    //! includes:
    //! * large signed and unsigned integers, named *gib int types* (256, 512,
    //!   1024-bit)
    //! * custom sub-8 bit unsigned integers, named *small int types (5-, 6-, 7-bit)
    //! * 24-bit signed integer.
    //!
    //! The functions here are designed to be fast.

    pub use amplify_num::*;
    #[cfg(feature = "apfloat")]
    pub use amplify_apfloat as apfloat;
}

pub use crate::as_any::AsAny;
pub use crate::bipolar::Bipolar;
pub use crate::strategy::Holder;
pub use crate::wrapper::Wrapper;
pub use crate::slice32::Slice32;
pub use crate::dumb_default::DumbDefault;
#[cfg(feature = "serde")]
pub use crate::to_serde_string::{ToYamlString, ToJsonString, ToTomlString};
#[cfg(all(feature = "std", feature = "derive"))]
pub use crate::io_error::IoError;
