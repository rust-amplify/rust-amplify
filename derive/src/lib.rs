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
    dead_code
)]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

#[macro_use]
mod util;

mod as_any;
mod display;
mod error;
mod from;
mod getters;
mod wrapper;

use proc_macro::TokenStream;
use syn::DeriveInput;

/// # Usage
///
/// 1. Generate [`Display`] descriptions using other formatting trait:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[derive(Display, Debug)]
///     #[display(Debug)]
///     enum Some {
///         Once,
///         Twice(u8)
///     }
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
///
///     pub trait ToSpecialString {
///         fn to_special_string(&self) -> String {
///             s!("Special string")
///         }
///     }
///
///     #[derive(Display)]
///     #[display(Some::to_special_string)]
///     struct Some { uint: u32, int: i32 };
///     impl ToSpecialString for Some {}
///
///     assert_eq!(
///         format!("{}", Int { uint: 2 }),
///         s!("Integer representation")
///     );
///
///     #[derive(Display)]
///     #[display(some_fmt)]
///     enum Enum { Once(u8), Twice };
///     fn some_fmt(_: &Enum) -> String { s!("Some") }
///     assert_eq!(format!("{}", Enum::Once(3)), s!("Some"))
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
///     /// Example enum with doc comments converted into display
///     #[derive(Display)]
///     #[display(doc_comments)]
///     enum Variants {
///         /// Letter A.
///         /// Multiline comments are also working, but joined together
///         ///
///         /// Empty line is replaced with line break
///         /// \nYou may also use this way
///         /// \n
///         /// The above will still result in a single line break
///         A,
///         /// Letter B
///         B,
///         /// This comment is ignored
///         #[display("Letter C")]
///         C,
///         /// Letter {0}
///         Letter(String),
///         /// You can omit parameters and just have a normal doc comment
///         Number(u8),
///         /// ... for variants with named fields as well
///         Named { some: String }
///     };
///
///     assert_eq!(
///         format!("{}", Variants::A),
///         "Letter A. Multiline comments are also working, but joined \
///         together\nEmpty line is replaced with line break\nYou may also use \
///         this way\nThe above will still result in a single line break"
///     );
///     assert_eq!(format!("{}", Variants::C), "Letter C");
///     assert_eq!(format!("{}", Variants::Letter(s!("K"))), "Letter K");
///    ```
///    You can also mix in this mode with other fors of display tags on a
///    specific options; in this case doc comments are ignored
/// 6. Support of unit structs and newtypes:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     /// Some unit struct
///     #[derive(Clone, Debug, Display, Error)]     
///     #[display(doc_comments)]
///     pub struct UnitStruct;
///
///     /// displaying the wrapped type data: '{0}'.
///     #[derive(Clone, PartialEq, Eq, Debug, Display)]
///     #[display(doc_comments)]
///     pub struct NewType(pub String);
///    ```
/// 7. Print the name of enum variant in lowercase/uppercase:
///    ```
///     # #[macro_use] extern crate amplify_derive;
///     #[derive(Display)]
///     #[display(lowercase)]
///     enum Message {
///         Quit,
///         Move { x: i32, y: i32 },
///         Write(String),
///         ChangeColor(i32, i32, i32),
///     }
///
///     #[derive(Display)]
///     #[display(uppercase)]
///     enum Event {
///         Init,
///         Load(Message),
///     }
///
///
///     assert_eq!(format!("{}", Message::Quit), "quit");
///     assert_eq!(format!("{}", Message::Move{ x: 1, y: 2 }),
///         "move { x: 1, y: 2 }");
///     assert_eq!(format!("{}", Message::Write(String::from("msg"))),
///         "write(msg)");
///     assert_eq!(format!("{}", Message::ChangeColor(255, 0, 0)),
///         "changecolor(255, 0, 0)");
///     assert_eq!(format!("{}", Event::Init), "INIT");
///     assert_eq!(format!("{}", Event::Load(Message::ChangeColor(0, 255, 0))),
///         "LOAD(changecolor(0, 255, 0))");
///    ```
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
///     #[display(inner)]
///     Inner {
///         a: String,
///     },
///
///     Unnamed(u16),
///
///     // NB: Use indexes for tuple values
///     #[display("Custom{0}")]
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
/// #[display("{0}")]
/// struct Tuple(u8);
///
/// #[derive(Clone, Copy, Debug, Display)]
/// #[display(inner)] // `inner` is synonym to "{0}"
/// struct Tuple2(u8);
///
/// assert_eq!(format!("{}", Tuple(5)), format!("{}", Tuple2(5)))
/// ```
///
/// Using inner enum variant representation, defaulting to the variant name
/// if the variant does not have inner data:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// #[derive(Clone, Copy, Debug, Display)]
/// #[display(inner)] // `inner` is synonym to "{0}"
/// enum Variants {
///     First,
///     Second,
///     WithData(u8),
///     WithComplexData(IpAddr),
/// };
///
/// assert_eq!(Variants::First.to_string(), "First");
/// assert_eq!(Variants::WithData(5).to_string(), "5");
/// assert_eq!(
///     Variants::WithComplexData(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))).to_string(),
///     "127.0.0.1"
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
///     /// Zero division with {0}
///     ZeroDivision(u16),
/// }
///
/// assert_eq!(format!("{}", Error::Io), "I/O operation error");
/// assert_eq!(format!("{}", Error::Overflow), "Math overflow");
/// assert_eq!(
///     format!("{}", Error::ZeroDivision(2)),
///     "Zero division with 2"
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

/// Trait `amplify::AsAny` allows simple conversion of any type into a
/// generic "thick" pointer `&dyn Any` (see [`::core::any::Any`]), that can be
/// later converted back to the original type with a graceful failing for all
/// other conversions. `AsAny` derive macro allows to implement this trait for
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

/// Derives getter methods for structures. The return type and naming of the
/// methods depends on the provided attribute arguments.
///
/// # Attribute `#[getter(...)]`
///
/// Macro is provided with `#[getter]` attribute, which may be used on both
/// type and field level. See following sections describing its arguments
///
/// ## Arguments
///
/// ### Method derivation arguments
/// Method derivation arguments define which forms of methods should be derived.
/// Applicable both at the type level, where it defines a set of derived methods
/// for all fields (unless they are overrided on the field level) – or on the
/// field level, where it overrides/replaces the default set of methods with a
/// new one.
///
/// Attribute takes a list of arguments in form of verbatim literals:
/// - `as_copy`: derives methods returning copy of the field value. Will error
///   at compile time on types which does not implement `Copy`
/// - `as_clone`: derives methods returning cloned value; will conflict with
///   `as_copy`. Errors at compile time on types which does not implement
///   `Clone`.
/// - `as_ref`: derives method returning reference. If provided together with
///   either `as_copy` or `as_clone`, method name returning reference is
///   suffixed with `_ref`; otherwise the base name is used (see below)
/// - `as_mut`: derives method returning mutable reference. Method name is
///   suffixed with `_mut`
/// - `all`: equivalent to `as_clone, as_ref, as_mut`
///
/// **Can be used**: at type and field level
///
/// **Defaults to**: `as_ref`
///
/// ### `#[getter(skip)]`
/// Skips derivation of a all gettter methods for this field
///
/// ### `#[getter(prefix = "...")]`
/// Defines prefix added to all derived getter method names.
///
/// **Defaults to**: none (no prefix added)
///
/// **Can be used**: at type level
///
/// ### `#[getter(base_name = "...")]`
/// Defines base name for the getter method. Base name is prefixed with prefix
/// from a type-level getter `prefix` attribute (if the one is specified) and
/// suffix, which is method-specific (see `methods` argument description above).
///
/// **Defaults to**: field name
///
/// **Can be used**: at field level
///
/// # Errors
///
/// Enums and units are not supported; attempt to derive `Getters` on them will
/// result in a compile-time error.
///
/// Deriving getters on unit structs and structs with unnamed fields (tupe
/// structs) is not supported (since it's meaningless), and results in a error.
///
/// Additionally to these two cases, macro errors on argument inconsistencies,
/// as described in the argument-specific sections.
///
/// # Examples
///
/// Basic use:
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Getters, Default)]
/// struct One {
///     vec: Vec<u8>,
///     defaults: String,
///     #[getter(as_copy)]
///     pub flag: bool,
///     #[getter(as_copy)]
///     pub(self) field: u8,
/// }
///
/// let mut one = One::default();
/// assert_eq!(one.vec(), &Vec::<u8>::default());
/// assert_eq!(one.defaults(), "");
/// assert_eq!(one.flag(), false);
/// assert_eq!(one.field(), 0);
/// ```
///
/// Important, that field-level arguments to override struct-level arguments:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Getters, Default)]
/// #[getter(as_copy)]
/// struct Other {
///     #[getter(as_ref)]
///     vec: Vec<u8>,
///     #[getter(as_clone)]
///     defaults: String,
///     pub flag: bool,
///     pub(self) field: u8,
/// }
///
/// let mut other = Other::default();
/// assert_eq!(other.vec(), &Vec::<u8>::default());
/// assert_eq!(other.defaults(), String::from(""));
/// ```
///
/// Advanced use: please pay attention that `as_mut` on a struct level is not
/// removed by the use of `as_copy` at field level.
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// #[derive(Getters, Default)]
/// #[getter(as_mut, prefix = "get_")]
/// struct One {
///     /// Contains byte representation of the data
///     #[getter(all, base_name = "bytes")]
///     vec: Vec<u8>,
///
///     defaults: String,
///
///     #[getter(as_copy)]
///     pub flag: bool,
///
///     #[getter(skip)]
///     pub(self) field: u8,
/// }
///
/// let mut one = One::default();
/// assert_eq!(one.get_bytes_ref(), &Vec::<u8>::default());
/// *one.get_bytes_mut() = vec![0, 1, 2];
/// assert_eq!(one.get_defaults(), "");
/// assert_eq!(one.get_defaults_mut(), "");
/// assert_eq!(one.get_bytes(), vec![0, 1, 2]);
/// assert_eq!(one.get_flag(), bool::default());
/// assert_eq!(one.get_flag_mut(), &mut bool::default());
/// let flag = one.get_flag_mut();
/// *flag = true;
/// assert_eq!(one.get_flag(), true);
/// assert_eq!(one.flag, one.get_flag());
/// // method does not exist: assert_eq!(one.get_field(), u8::default());
/// ```
///
/// this will end up in the following generated code:
/// ```
/// # struct One {
/// #    vec: Vec<u8>,
/// #    pub flag: bool,
/// #    pub(self) field: u8,
/// # }
///
/// impl One {
///     #[doc = "Method cloning [`One::vec`] field.\n"]
///     #[doc = " Contains byte representation of the data"]
///     #[inline]
///     pub fn get_bytes(&self) -> Vec<u8> {
///         self.vec.clone()
///     }
///
///     #[doc = "Method borrowing [`One::vec`] field.\n"]
///     #[doc = " Contains byte representation of the data"]
///     #[inline]
///     pub fn get_bytes_ref(&self) -> &Vec<u8> {
///         &self.vec
///     }
///
///     #[doc = "Method returning mutable borrow of [`One::vec`] field.\n"]
///     #[doc = " Contains byte representation of the data"]
///     #[inline]
///     pub fn get_bytes_mut(&mut self) -> &mut Vec<u8> {
///         &mut self.vec
///     }
///
///     #[doc = "Method returning copy of [`One::flag`] field.\n"]
///     #[inline]
///     pub fn get_flag(&self) -> bool {
///         self.flag
///     }
///
///     #[doc = "Method returning mutable borrow of [`One::flag`] field.\n"]
///     #[inline]
///     pub fn get_flag_mut(&mut self) -> &mut bool {
///         &mut self.flag
///     }
/// }
/// ```
#[proc_macro_derive(Getters, attributes(getter))]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    getters::derive(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Creates rust new type wrapping existing type. Can be used in structures
/// containing multiple named or unnamed fields; in this case the field you'd
/// like to wrap should be marked with `#[wrap]` attribute; otherwise the first
/// field is assumed to be the wrapped one.
///
/// NB: You have to use `derive(From)` in order foe Wrapper to work properly.
/// Also, in case of multiple fields, each non-wrapped field type must implement
/// `Default` trait.
///
/// Supports automatic implementation of the following traits:
/// * `amplify::Wrapper`
/// * [`AsRef`]
/// * [`core::borrow::Borrow`]
///
/// You can implement additional derives, it they are implemented for the wrapped
/// type, using `#[wrapper()]` proc macro:
/// 1. Reference access to the inner type:
///    * `Deref` for implementing [`core::ops::Deref`]
///    * `BorrowSlice` for implementing [`core::borrow::Borrow`]`<[Self::Inner]>`
/// 2. Formatting:
///    * `FromStr` for implementing [`core::str::FromStr`]
///    * `Debug` for implementing [`core::fmt::Debug`]
///    * `Display` for implementing [`core::fmt::Display`]
///    * `FromHex` for implementing [`amplify::hex::FromHex`]
///    * `LowerHex` for implementing [`core::fmt::LowerHex`]
///    * `UpperHex` for implementing [`core::fmt::UpperHex`]
///    * `LowerExp` for implementing [`core::fmt::LowerExp`]
///    * `UpperExp` for implementing [`core::fmt::UpperExp`]
///    * `Octal` for implementing [`core::fmt::Octal`]
/// 3. Indexed access to the inner type:
///    * `Index` for implementing [`core::ops::Index`]`<usize>`
///    * `IndexRange` for implementing
///      [`core::ops::Index`]`<`[`core::ops::Range`]`<usize>>`
///    * `IndexTo` for implementing
///      [`core::ops::Index`]`<`[`core::ops::RangeTo`]`<usize>>`
///    * `IndexFrom` for implementing
///      [`core::ops::Index`]`<`[`core::ops::RangeFrom`]`<usize>>`
///    * `IndexInclusive` for implementing
///      [`core::ops::Index`]`<`[`core::ops::RangeInclusive`]`<usize>>`
///    * `IndexToInclusive` for implementing
///      [`core::ops::Index`]`<`[`core::ops::RangeToInclusive`]`<usize>>`
///    * `IndexFull` for implementing
///      [`core::ops::Index`]`<`[`core::ops::RangeFrom`]`<usize>>`
/// 4. Arithmetic operations:
///    * `Neg` for implementing [`core::ops::Neg`]
///    * `Add` for implementing [`core::ops::Add`]
///    * `Sub` for implementing [`core::ops::Sub`]
///    * `Mul` for implementing [`core::ops::Mul`]
///    * `Div` for implementing [`core::ops::Div`]
///    * `Rem` for implementing [`core::ops::Rem`]
/// 5. Boolean and bit-wise operations:
///    * `Not` for implementing [`core::ops::Not`]
///    * `BitAnd` for implementing [`core::ops::BitAnd`]
///    * `BitOr` for implementing [`core::ops::BitOr`]
///    * `BitXor` for implementing [`core::ops::BitXor`]
///    * `Shl` for implementing [`core::ops::Shl`]
///    * `Shr` for implementing [`core::ops::Shr`]
///
/// There are shortcuts for derivations:
/// * `#[wrapper(Hex)]` will derive both `LowerHex`, `UpperHex` and `FromHex`;
/// * `#[wrapper(Exp)]` will derive both `LowerExp` and `UpperExp`;
/// * `#[wrapper(NumberFmt)]` will derive all number formatting traits
///    (`LowerHex`, `UpperHex`, `LowerExp`, `UpperExp`, `Octal`);
/// * `#[wrapper(RangeOps)]` will derive all index traits working with ranges
///    (`IndexRange`, `IndexTo`, `IndexFrom`, `IndexInclusive`,
///    `IndexToInclusive`, `IndexFull`);
/// * `#[wrapper(MathOps)]` will derive all arithmetic operations
///    (`Neg`, `Add`, `Sub`, `Mul`, `Div`, `Rem`);
/// * `#[wrapper(BoolOps)]` will derive all boolean operations
///    (`Not`, `BitAnd`, `BitOr`, `BitXor`);
/// * `#[wrapper(BitOps)]` will derive all boolean operations *and bit shifts*
///    (`Not`, `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`).
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
/// #[wrapper(MathOps, BitOps)]
/// struct Int64(i64);
/// ```
///
/// More complex wrapper with multiple unnamed fields:
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// # use std::collections::HashMap;
/// # use std::fmt::Debug;
/// use std::marker::PhantomData;
/// use amplify::Wrapper;
///
/// #[derive(Clone, Wrapper, Default, From)]
/// #[wrapper(Debug)]
/// struct Wrapped<T, U>(
///     #[wrap]
///     #[from]
///     HashMap<usize, Vec<U>>,
///     PhantomData<T>,
/// )
/// where
///     U: Sized + Clone + Debug;
///
/// let w = Wrapped::<(), u8>::default();
/// assert_eq!(w.into_inner(), HashMap::<usize, Vec<u8>>::default());
/// ```
///
/// Wrappers for indexable types
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// use amplify::Wrapper;
///
/// #[derive(Wrapper, From)]
/// #[wrapper(Index, RangeOps)]
/// struct VecNewtype(Vec<u8>);
/// ```
#[proc_macro_derive(Wrapper, attributes(wrap, wrapper, amplify_crate))]
pub fn derive_wrapper(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    wrapper::inner(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derives [`WrapperMut`] and allows deriving other traits accessing the
/// wrapped type which require mutable access to the inner type. Requires that
/// the type already implements `amplify::Wrapper`.
///
/// Supports automatic implementation of the following traits:
/// * `amplify::WrapperMut`
/// * [`AsMut`]
/// * [`core::borrow::BorrowMut`]
///
/// You can implement additional derives, it they are implemented for the wrapped
/// type, using `#[wrapper()]` proc macro:
/// 1. Reference access to the inner type:
///    * `DerefMut` for implementing [`core::ops::DerefMut`]
///    * `BorrowSliceMut` for implementing
///      [`core::borrow::BorrowMut`]`<[Self::Inner]>`
/// 2. Indexed access to the inner type:
///    * `IndexMut` for implementing [`core::ops::IndexMut`]`<usize>`
///    * `IndexRangeMut` for implementing
///      [`core::ops::IndexMut`]`<`[`core::ops::Range`]`<usize>>`
///    * `IndexToMut` for implementing
///      [`core::ops::IndexMut`]`<`[`core::ops::RangeTo`]`<usize>>`
///    * `IndexFromMut` for implementing
///      [`core::ops::IndexMut`]`<`[`core::ops::RangeFrom`]`<usize>>`
///    * `IndexInclusiveMut` for implementing
///      [`core::ops::IndexMut`]`<`[`core::ops::RangeInclusive`]`<usize>>`
///    * `IndexToInclusiveMut` for implementing
///      [`core::ops::IndexMut`]`<`[`core::ops::RangeToInclusive`]`<usize>>`
///    * `IndexFullMut` for implementing
///      [`core::ops::IndexMut`]`<`[`core::ops::RangeFrom`]`<usize>>`
/// 3. Arithmetic operations:
///    * `AddAssign` for implementing [`core::ops::AddAssign`]
///    * `SubAssign` for implementing [`core::ops::SubAssign`]
///    * `MulAssign` for implementing [`core::ops::MulAssign`]
///    * `DivAssign` for implementing [`core::ops::DivAssign`]
///    * `RemAssign` for implementing [`core::ops::RemAssign`]
/// 4. Boolean and bit-wise operations:
///    * `BitAndAssign` for implementing [`core::ops::BitAndAssign`]
///    * `BitOrAssign` for implementing [`core::ops::BitOrAssign`]
///    * `BitXorAssign` for implementing [`core::ops::BitXorAssign`]
///    * `ShlAssign` for implementing [`core::ops::ShlAssign`]
///    * `ShrAssign` for implementing [`core::ops::ShrAssign`]
///
/// There are shortcuts for derivations:
/// * `#[wrapper(RangeMut)]` will derive all index traits working with
///    ranges (`IndexRangeMut`, `IndexToMut`, `IndexFromMut`,
///    `IndexInclusiveMut`, `IndexToInclusiveMut`, `IndexFullMut`);
/// * `#[wrapper(MathAssign)]` will derive all arithmetic operations
///    (`AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `RemAssign`);
/// * `#[wrapper(BoolAssign)]` will derive all boolean operations
///    (`BitAndAssign`, `BitOrAssign`, `BitXorAssign`);
/// * `#[wrapper(BitAssign)]` will derive all boolean operations
///    *and bit shifts* (`BitAndAssign`, `BitOrAssign`, `BitXorAssign`,
///    `ShlAssign`, `ShrAssign`);
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate amplify_derive;
/// use amplify::{Wrapper, WrapperMut};
///
/// #[derive(
///     Wrapper, WrapperMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, From, Debug,
///     Display,
/// )]
/// #[display(inner)]
/// #[wrapper(NumberFmt, MathOps, BoolOps)]
/// #[wrapper_mut(MathAssign, BitAssign)]
/// struct Int64(i64);
/// ```
#[proc_macro_derive(WrapperMut, attributes(wrap, wrapper_mut, amplify_crate))]
pub fn derive_wrapper_mut(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    wrapper::inner_mut(derive_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
