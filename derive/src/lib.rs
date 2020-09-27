// Rust language amplification derive library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Elichai Turkel <elichai.turkel@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![recursion_limit = "256"]
#![feature(try_find)]
#![allow(unused)]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate amplify;

mod as_any;
mod display;
mod getters;

use syn::export::TokenStream;
use syn::DeriveInput;

/// # Example
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Display)]
/// enum Test {
///     Some,
///
///     #[display = "OtherName"]
///     Other,
///
///     Named {
///         x: u8,
///     },
///
///     #[display = "Custom{x}"]
///     NamedCustom {
///         x: u8,
///     },
///     Unnamed(u16),
///
///     #[display = "Custom{_0}"]
///     UnnamedCustom(String),
/// }
///
/// assert_eq!(format!("{}", Test::Some), "Some");
/// assert_eq!(format!("{}", Test::Other), "OtherName");
/// assert_eq!(format!("{}", Test::Named { x: 1 }), "Named { .. }");
/// assert_eq!(format!("{}", Test::Unnamed(5)), "Unnamed(..)");
/// assert_eq!(format!("{}", Test::NamedCustom { x: 8 }), "Custom8");
/// assert_eq!(
///     format!("{}", Test::UnnamedCustom("Test".to_string())),
///     "CustomTest"
/// );
/// ```
#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    display::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_derive(AsAny)]
pub fn derive_as_any(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    as_any::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    getters::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
