// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2021 by
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

//! Amplifying Rust language capabilities: helper functions for creating proc
//! macro libraries
//!
//! # Examples
//!
//! `#[name]` - single form
//! `#[name = "literal"]` - optional single value
//! `#[name = TypeName]` - path value
//! `#[name("literal", TypeName, arg = value)]` - list of arguments

#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    missing_docs,
    dead_code,
    warnings
)]

#[macro_use]
extern crate quote;

mod attr;
mod cls;
mod error;
mod req;
mod val;

pub use error::Error;
pub use attr::{Attr, SingularAttr, ParametrizedAttr, ExtractAttr};
pub use cls::{LiteralClass, ValueClass, TypeClass};
pub use req::{ValueReq, ListReq, AttrReq, ArgReq};
pub use val::ArgValue;
