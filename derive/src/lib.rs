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

//! Amplifying Rust language capabilities: multiple generic trait
//! implementations, type wrappers, derive macros.

#![recursion_limit = "256"]
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
#[macro_use]
extern crate syn;

/// Macro producing [`Result::Err`] with [`syn::Error`] containing span
/// information from `$attr` (first) argument and formatted string describing
/// concrete error (description is taken from `$msg` second macro argument) and
/// providing an example `$example` (third macro argument) of how the macro
/// should be used.
macro_rules! attr_err {
    ($attr:expr, $msg:tt) => {
        attr_err!($attr.span(), NAME, $msg, EXAMPLE);
    };
    ($name:expr, $msg:tt, $example:tt) => {
        attr_err!(::syn::export::Span::call_site(), $name, $msg, $example);
    };
    ($attr:expr, $name:expr, $msg:tt, $example:tt) => {
        ::syn::Error::new(
            $attr.span(),
            format!(
                "Attribute `#[{}]`: {}\nExample use: {}",
                $name, $msg, $example
            ),
        );
    };
}

mod as_any;
mod display;
mod error;
mod from;
mod getters;
mod wrapper;

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
///     assert_eq!(format!("{}", Point { x: 0, y: 1 }), "(0, 1)");
///    ```
/// 4. Support for alternative formatting with `alt` parameter:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[derive(Display)]
///     #[display("({x}, {y})", alt = "{x}:{y}")]
///     struct Point { x: u32, y: u32 }
///     assert_eq!(format!("{}", Point { x: 0, y: 1 }), "(0, 1)");
///     assert_eq!(format!("{:#}", Point { x: 0, y: 1 }), "0:1");
///    ```
/// 5. Use of doc comments for descrition representation. In this case doc
///    comments may also contain formatting like in the case 3:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[macro_use] extern crate amplify;
///
///     #[derive(Display)]
///     #[display(doc_comments)]
///     enum Variants {
///         /// Letter A
///         /// Multiline comments are also working
///         A,
///         /// Letter B
///         B,
///         /// This comment is ignored
///         #[display("Letter C")]
///         C,
///         /// Letter {_0}
///         Letter(String),
///         /// You can omit parameters and just have a normal doc comment
///         Number(u8),
///         /// ... for variants with named fields as well
///         Named { some: String }
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
/// #[derive(Display)]
/// enum Test {
///     Some,
///
///     #[display("OtherName")]
///     Other,
///
///     /// Document comment working as display string
///     Commented,
///
///     Named {
///         x: u8,
///     },
///
///     #[display("Custom{x}", alt = "this is alternative")]
///     NamedCustom {
///         x: u8,
///     },
///
///     Unnamed(u16),
///
///     // NB: Use `_`-prefixed indexes for tuple values
///     #[display("Custom{_0}")]
///     UnnamedCustom(String),
/// }
///
/// assert_eq!(format!("{}", Test::Some), "Some");
/// assert_eq!(format!("{}", Test::Other), "OtherName");
/// assert_eq!(format!("{}", Test::Named { x: 1 }), "Named { .. }");
/// assert_eq!(format!("{}", Test::Unnamed(5)), "Unnamed(..)");
/// assert_eq!(format!("{}", Test::NamedCustom { x: 8 }), "Custom8");
/// assert_eq!(
///     format!("{:#}", Test::NamedCustom { x: 8 }),
///     "this is alternative"
/// );
/// assert_eq!(
///     format!("{}", Test::UnnamedCustom("Test".to_string())),
///     "CustomTest"
/// );
/// ```
///
/// Use with tuple types:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Clone, Copy, Debug, Display)]
/// #[display("{_0}")]
/// struct Tuple(u8);
///
/// #[derive(Clone, Copy, Debug, Display)]
/// #[display(inner)] // `inner` is synonym to "{_0}"
/// struct Tuple2(u8);
///
/// assert_eq!(format!("{}", Tuple(5)), format!("{}", Tuple2(5)))
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
/// # Example
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

/// Implements [`From`] trait for the whole entity and/or its separate fields.
/// Works well with `#[derive(Error)]` and, in many cases may require
/// [`Default`] implementation (for details, pls see Examples below)
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #
/// #[derive(From, Default)]
/// #[from(::std::io::Error)]
/// // Structure may contain no parameters
/// pub struct IoErrorUnit;
///
/// #[derive(From, Default)]
/// #[from(::std::io::Error)] // When no explicit binding is given, structure must implement `Default`
/// pub struct IoError {
///     details: String,
///
///     #[from]
///     kind: IoErrorUnit,
/// }
///
/// #[derive(From)]
/// pub enum Error {
///     // You can specify multiple conversions with separate attributes
///     #[from(::std::io::Error)]
///     #[from(IoError)]
///     Io,
///
///     #[from]
///     Format(::std::fmt::Error),
///
///     #[from]
///     WithFields { details: ::std::str::Utf8Error },
///
///     MultipleFields {
///         // ...and you can also covert error type
///         #[from(IoErrorUnit)]
///         // rest of parameters must implement `Default`
///         io: IoError,
///         details: String,
///     },
/// }
///
/// #[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, From)]
/// pub struct Wrapper(u32, i16);
/// ```
///
/// If you use rust nightly and `#![feature(never_type)]` for [`!`], you can
/// even do the following:
/// ```ignore
/// #![feature(never_type)]
///
/// #[macro_use]
/// extern crate amplify_derive;
///
/// #[derive(From)]
/// pub enum Error {
///     // ... other error types
///     #[from(!)]
///     NeverType,
/// }
///
/// # fn main () {
/// # }
/// ```
#[proc_macro_derive(From, attributes(from))]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    from::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Trait [`amplify::AsAny`] allows simple conversion of any type into a generic
/// "thick" pointer `&dyn Any` (see [`::core::any::Any`]), that can be later
/// converted back to the original type with a graceful failing for all other
/// conversions. `AsAny` derive macro allows to implement this trait for
/// arbitrary time without much hussle:
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// extern crate amplify;
/// use amplify::AsAny;
///
/// #[derive(AsAny, Copy, Clone, PartialEq, Eq, Debug)]
/// struct Point {
///     pub x: u64,
///     pub y: u64,
/// }
///
/// #[derive(AsAny, PartialEq, Debug)]
/// struct Circle {
///     pub radius: f64,
///     pub center: Point,
/// }
///
/// let mut point = Point { x: 1, y: 2 };
/// let point_ptr = point.as_any();
///
/// let mut circle = Circle {
///     radius: 18.,
///     center: point,
/// };
/// let circle_ptr = circle.as_any();
///
/// assert_eq!(point_ptr.downcast_ref(), Some(&point));
/// assert_eq!(circle_ptr.downcast_ref(), Some(&circle));
/// assert_eq!(circle_ptr.downcast_ref::<Point>(), None);
///
/// let p = point_ptr.downcast_ref::<Point>().unwrap();
/// assert_eq!(p.x, 1)
/// ```
#[proc_macro_derive(AsAny)]
pub fn derive_as_any(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    as_any::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Creates getter methods matching field names for all fields within a
/// structure (including public and private fields). Getters return reference
/// types.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Getters, Default)]
/// struct One {
///     a: Vec<u8>,
///     pub b: bool,
///     pub(self) c: u8,
/// }
///
/// let one = One::default();
/// assert_eq!(one.a(), &Vec::<u8>::default());
/// assert_eq!(one.b(), &bool::default());
/// assert_eq!(one.c(), &u8::default());
/// ```
#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    getters::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Creates rust new type wrapping existing type. Can be used in sturctures
/// containing multiple named or unnamed fields; in this case the field you'd
/// like to wrap should be marked with `#[wrap]` attribute; otherwise the first
/// field is assumed to be the wrapped one.
///
/// NB: You have to use `derive(From)` in order foe Wrapper to work properly.
/// Also, in case of multiple fields, each non-wrapped field type must implement
/// `Default` trait.
///
/// Supports automatic implementation of the following traits:
/// * [`amplify::Wrapper`]
/// * [`AsRef`]
/// * [`AsMut`]
/// * [`Borrow`]
/// * [`BorrowMut`]
/// * [`Deref`]
/// * [`DerefMut`]
///
/// You can implement additonal derives, it they are implemented for the wrapped
/// type, using `#[wrapper()]` proc macro:
/// * [`LowerHex`]
/// * [`UpperHex`]
/// * [`LowerExp`]
/// * [`UpperExp`]
/// * [`Octal`]
/// * [`Index`]
/// * [`IndexMut`]
/// * [`Neg`]
/// * [`Not`]
/// * [`Add`]
/// * [`AddAssign`]
/// * [`Sub`]
/// * [`SubAssign`]
/// * [`Mul`]
/// * [`MulAssign`]
/// * [`Div`]
/// * [`DivAssign`]
/// * [`Rem`]
/// * [`RemAssign`]
/// * [`Shl`]
/// * [`ShlAssign`]
/// * [`Shr`]
/// * [`ShrAssign`]
/// * [`BitAnd`]
/// * [`BitAndAssign`]
/// * [`BitOr`]
/// * [`BitOrAssign`]
/// * [`BitXor`]
/// * [`BitXorAssign`]
///
/// Other traits, such as [`PartialEq`], [`Eq`], [`PartialOrd`], [`Ord`],
/// [`Hash`] can be implemented using standard `#[derive]` attribute in the
/// same manner as [`Default`], [`Debug`] and [`From`]
///
/// # Example
///
/// Simple wrapper:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// use amplify::Wrapper;
///
/// #[derive(
///     Wrapper, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, From, Debug, Display,
/// )]
/// #[display(inner)]
/// #[wrapper(LowerHex, UpperHex, Octal)]
/// #[wrapper(Neg, Add, Sub, Div, Mul, Rem)]
/// #[wrapper(AddAssign, SubAssign, DivAssign, MulAssign, RemAssign)]
/// #[wrapper(Not, Shl, Shr, BitAnd, BitOr, BitXor)]
/// #[wrapper(ShlAssign, ShrAssign, BitAndAssign, BitOrAssign, BitXorAssign)]
/// struct Int64(i64);
/// ```
///
/// More complex wrapper with multiple unnamed fields:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// # use std::collections::HashMap;
/// use std::marker::PhantomData;
/// use amplify::Wrapper;
///
/// #[derive(Clone, Wrapper, Default, From, Debug)]
/// struct Wrapped<T, U>(
///     #[wrap]
///     #[from]
///     HashMap<usize, Vec<U>>,
///     PhantomData<T>,
/// )
/// where
///     U: Sized + Clone;
///
/// let w = Wrapped::<(), u8>::default();
/// assert_eq!(w.into_inner(), HashMap::<usize, Vec<u8>>::default());
/// ```
#[proc_macro_derive(Wrapper, attributes(wrap, wrapper))]
pub fn derive_wrapper(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    wrapper::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
