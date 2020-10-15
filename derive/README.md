# Rust Amplify Library: derive macros
[![crates.io](https://meritbadge.herokuapp.com/amplify_derive)](https://crates.io/crates/amplify_derive)
[![Docs](https://docs.rs/amplify_derive/badge.svg)](https://docs.rs/amplify_derive)
![Build](https://github.com/LNP-BP/rust-amplify/workflows/Build/badge.svg)
![Tests](https://github.com/LNP-BP/rust-amplify/workflows/Tests/badge.svg)
![Lints](https://github.com/LNP-BP/rust-amplify/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/LNP-BP/rust-amplify/branch/master/graph/badge.svg)](https://codecov.io/gh/LNP-BP/rust-amplify)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Amplifying Rust language capabilities: multiple generic trait implementations, 
type wrappers, derive macros.

This is a part of Rust language amplification library providing required derive
macros.

Minimum supported rust compiler version (MSRV): 1.41.1

## Overview

- [Display](#display-derive)
- [From](#from-derive)
- [Error](#error-derive)
- [Getters](#getters-derive)
- [Wrapper](#wrapper-derive)
- [AsAny](#asany-derive)

## Display derive 

1. Generate [`Display`] descriptions using other formatting trait:
   ```rust
    #[derive(Display, Debug)]
    #[display(Debug)]
    struct Some { /* ... */ }
   ```
2. Use existing function for displaying descriptions:
   ```rust
    #[derive(Display)]
    #[display(Int::print)]
    union Int { uint: u32, int: i32 };
   
    impl Int {
        pub fn print(&self) -> String {
            s!("Integer representation")
        }
    }
   ```
   Formatting function must return [`String`] and take a single `self`
   argument (if you need formatting with streamed output, use one of
   existing formatting traits as shown in pt. 1).
3. Custom format string:
   ```rust
    #[derive(Display)]
    #[display("({x}, {y})")]
    struct Point { x: u32, y: u32 }
   ```
4. Use of doc comments for descrition representation. In this case doc
   comments may also contain formatting like in the case 3:
   ```rust
    #[macro_use] extern crate amplify;

    #[derive(Display)]
    #[display(doc_comments)]
    enum Variants {
        /// Letter A
        A,

        /// Letter B
        B,

        /// This comment is ignored
        #[display("Letter C")]
        C,

        /// Letter {_0}
        Letter(String)
    };

    assert_eq!(format!("{}", Variants::C), "Letter C");
    assert_eq!(format!("{}", Variants::Letter(s!("K"))), " Letter K");
   ```
   You can also mix in this mode with other fors of display tags on a
   specific options; in this case doc comments are ignored

# Example

Advanced use with enums:
```rust
#[derive(Debug, Display)]
#[display(Debug)]
enum Test {
    Some,

    #[display = "OtherName"]
    Other,

    Named {
        x: u8,
    },

    #[display = "Custom{x}"]
    NamedCustom {
        x: u8,
    },

    Unnamed(u16),

    // NB: Use `_`-prefixed indexes for tuple values
    #[display = "Custom{_0}"]
    UnnamedCustom(String),
}
```

## Error derive

Error derive macro works to the full extend only when other derive macros
are used. With `#[derive(Display)]` and `[display(doc_comments)]` it uses
doc comments for generating error descriptions; with `#[derive(From)]` it
may automatically implement transofrations from other error types.

```rust
#[derive(Debug, Display, Error)]
#[display(doc_comments)]
enum Error {
    /// I/O operation error
    Io,
    /// Math overflow
    Overflow,
    /// Zero division with {_0}
    ZeroDivision(u16),
}
```

## From derive

Implements [`From`] trait for the whole entity and/or its separate fields.
Works well with `#[derive(Error)]` and, in many cases may require
[`Default`] implementation (for details, pls see Examples below)

# Examples

```rust
#[derive(From, Default)]
#[from(::std::io::Error)]
// Structure may contain no parameters
pub struct IoErrorUnit;

#[derive(From, Default)]
#[from(::std::io::Error)] // When no explicit binding is given, structure must implement `Default`
pub struct IoError {
    details: String,

    #[from]
    kind: IoErrorUnit,
}

#[derive(From)]
pub enum Error {
    // You can specify multiple conversions with separate attributes
    #[from(::std::io::Error)]
    #[from(IoError)]
    Io,

    #[from]
    Format(::std::fmt::Error),

    #[from]
    WithFields { details: ::std::str::Utf8Error },

    MultipleFields {
        // ...and you can also covert error type
        #[from(IoErrorUnit)]
        // rest of parameters must implement `Default`
        io: IoError,
        details: String,
    },
}

#[derive(From)]
pub struct Wrapper(u32, i16);
```

## Wrapper derive

Creates rust new type wrapping existing type. Can be used in sturctures
containing multiple named or unnamed fields; in this case the field you'd
like to wrap should be marked with `#[wrap]` attribute; otherwise the first
field is assumed to be the wrapped one.

Use with multiple fileds requires that you do `From` and `Default` derive
on the main structure.

Supports automatic implementation of the following traits:
* `amplify::Wrapper`
* `AsRef`
* `AsMut`
* `Borrow`
* `BorrowMut`
* `Deref`
* `DerefMut`

Complete usage of this derive macro is possible only with nightly rust
compiler with `trivial_bounds` feature gate set for the crate and `nightly`
feature set. This will give you an automatic implementation for additional
traits, it they are implemented for the wrapped type:
* `Display`
* `LowerHex`
* `UpperHex`
* `LowerExp`
* `UpperExp`
* `Octal`
* `Index`
* `IndexMut`
* `Add`
* `AddAssign`
* `Sub`
* `SubAssign`
* `Mul`
* `MulAssign`
* `Div`
* `DivAssign`

Other traits, such as `PartialEq`, `Eq`, `PartialOrd`, `Ord`,
`Hash` can be implemented using standard `#[derive]` attribute in the
same manner as `Default`, `Debug` and `From`

### Example

```rust
use std::marker::PhantomData;
use amplify::Wrapper;

#[derive(Clone, Wrapper, Default, From, Debug)]
struct Wrapped<T, U>(
    #[wrap]
    #[from]
    HashMap<usize, Vec<U>>,
    PhantomData<T>,
)
where
    U: Sized + Clone;

let w = Wrapped::<(), u8>::default();
assert_eq!(w.into_inner(), HashMap::<usize, Vec<u8>>::default());
```

## Getters derive

Creates getter methods matching field names for all fields within a
structure (including public and private fields). Getters return reference
types.

### Example

```
#[derive(Getters, Default)]
struct One {
    a: Vec<u8>,
    pub b: bool,
    pub(self) c: u8,
}

let one = One::default();
assert_eq!(one.a(), &Vec::<u8>::default());
assert_eq!(one.b(), &bool::default());
assert_eq!(one.c(), &u8::default());
```

## AsAny derive

Trait [`amplify::AsAny`] allows simple conversion of any type into a generic
"thick" pointer `&dyn Any` (see [`::core::any::Any`]), that can be later
converted back to the original type with a graceful failing for all other
conversions. `AsAny` derive macro allows to implement this trait for
arbitrary time without much hussle:

### Example

```
# #[macro_use] extern crate amplify_derive;
extern crate amplify;
use amplify::AsAny;

#[derive(AsAny, Copy, Clone, PartialEq, Eq, Debug)]
struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(AsAny, PartialEq, Debug)]
struct Circle {
    pub radius: f64,
    pub center: Point,
}

let mut point = Point { x: 1, y: 2 };
let point_ptr = point.as_any();

let mut circle = Circle {
    radius: 18.,
    center: point,
};
let circle_ptr = circle.as_any();

assert_eq!(point_ptr.downcast_ref(), Some(&point));
assert_eq!(circle_ptr.downcast_ref(), Some(&circle));
assert_eq!(circle_ptr.downcast_ref::<Point>(), None);

let p = point_ptr.downcast_ref::<Point>().unwrap();
assert_eq!(p.x, 1)
```
