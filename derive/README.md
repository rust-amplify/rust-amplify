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
- Getters
- AsAny

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
