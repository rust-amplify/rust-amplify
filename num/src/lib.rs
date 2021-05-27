// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
// Updated in 2020-2021 by
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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

mod bigint;
pub mod error;
#[cfg(feature = "hex")]
pub mod hex;
mod smallint;

pub use bigint::{u256, u512, u1024};
pub use smallint::{u1, u2, u3, u4, u5, u6, u7, u24};

// TODO: Create arbitrary precision types
// TODO: Move from using `u64` to `u128` for big int types
