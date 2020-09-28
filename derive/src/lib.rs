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
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate amplify;

/// Macro producing [`Result::Err`] with [`syn::Error`] containing span
/// information from `$attr` (first) argument and formatted string describing
/// concrete error (description is taken from `$msg` second macro argument) and
/// providing an example `$example` (third macro argument) of how the macro
/// should be used.
macro_rules! attr_err {
    ($name:expr, $msg:tt, $example:tt) => {
        attr_err!(::syn::export::Span::call_site(), $name, $msg, $example);
    };
    ($attr:expr, $name:expr, $msg:tt, $example:tt) => {
        ::syn::Error::new(
            $attr.span(),
            format!("Attribute {}: {}\nExample use: {}", $name, $msg, $example),
        );
    };
}

mod as_any;
mod display;
mod error;
mod getters;

use syn::export::TokenStream;
use syn::DeriveInput;

/// # Usage
///
/// 1. Generate [`Display`] descriptions using other formatting trait:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[derive(Display, Debug)]
///     #[display(Debug)]
///     struct Some { /* ... */ }
///    ```
/// 2. Use existing function for displaying descriptions:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[macro_use] extern crate amplify;
///
///     #[derive(Display)]
///     #[display(Int::print)]
///     union Int { uint: u32, int: i32 };
///     impl Int {
///         pub fn print(&self) -> String {
///             s!("Integer representation")
///         }
///     }
///    ```
///    Formatting function must return [`String`] and take a single `self`
///    argument (if you need formatting with streamed output, use one of
///    existing formatting traits as shown in pt. 1).
/// 3. Custom format string:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[derive(Display)]
///     #[display("({x}, {y})")]
///     struct Point { x: u32, y: u32 }
///    ```
/// 4. Use of doc comments for descrition representation. In this case doc
///    comments may also contain formatting like in the case 3:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[macro_use] extern crate amplify;
///
///     #[derive(Display)]
///     #[display(doc_comments)]
///     enum Variants {
///         /// Letter A
///         A,
///         /// Letter B
///         B,
///         /// This comment is ignored
///         #[display("Letter C")]
///         C,
///         /// Letter {_0}
///         Letter(String)
///     };
///
///     assert_eq!(format!("{}", Variants::C), "Letter C");
///     assert_eq!(format!("{}", Variants::Letter(s!("K"))), " Letter K");
///    ```
///    You can also mix in this mode with other fors of display tags on a
///    specific options; in this case doc comments are ignored
///
/// # Example
///
/// Advanced use with enums:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Debug, Display)]
/// #[display(Debug)]
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
///
///     Unnamed(u16),
///
///     // NB: Use `_`-prefixed indexes for tuple values
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

/// Error derive macro works to the full extend only when other derive macros
/// are used. With `#[derive(Display)]` and `[display(doc_comments)]` it uses
/// doc comments for generating error descriptions; with `#[derive(From)]` it
/// may automatically implement transofrations from other error types.
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Debug, Display, Error)]
/// #[display(doc_comments)]
/// enum Error {
///     /// I/O operation error
///     Io,
///     /// Math overflow
///     Overflow,
///     /// Zero division with {_0}
///     ZeroDivision(u16),
/// }
///
/// assert_eq!(format!("{}", Error::Io), " I/O operation error");
/// assert_eq!(format!("{}", Error::Overflow), " Math overflow");
/// assert_eq!(
///     format!("{}", Error::ZeroDivision(2)),
///     " Zero division with 2"
/// );
/// ```
#[proc_macro_derive(Error)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    error::inner(derive_input)
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
