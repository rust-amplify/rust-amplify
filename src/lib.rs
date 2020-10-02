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

#[cfg(feature = "async")]
#[macro_use]
extern crate async_trait;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

#[macro_use]
mod macros;
#[macro_use]
mod convert;
#[macro_use]
mod wrapper;

mod as_any;
mod bipolar;
#[cfg(feature = "std")]
pub mod internet;
#[cfg(feature = "serde")]
mod serde_helpers;
#[cfg(feature = "async")]
mod service;
mod strategy;

pub use crate::as_any::AsAny;
pub use crate::bipolar::Bipolar;
#[cfg(feature = "serde")]
pub use crate::serde_helpers::CowHelper;
#[cfg(feature = "async")]
pub use crate::service::{Service, TryService};
pub use crate::strategy::Holder;
pub use crate::wrapper::Wrapper;
