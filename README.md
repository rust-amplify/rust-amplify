# Rust Amplify Library

![Build](https://github.com/rust-amplify/rust-amplify/workflows/Build/badge.svg)
![Tests](https://github.com/rust-amplify/rust-amplify/workflows/Tests/badge.svg)
![Lints](https://github.com/rust-amplify/rust-amplify/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/rust-amplify/rust-amplify/branch/master/graph/badge.svg)](https://codecov.io/gh/rust-amplify/rust-amplify)

[![crates.io](https://img.shields.io/crates/v/amplify)](https://crates.io/crates/amplify)
[![Docs](https://docs.rs/amplify/badge.svg)](https://docs.rs/amplify)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Amplifying Rust language capabilities: multiple generic trait implementations, 
type wrappers, derive macros. Tiny library with zero non-optional dependencies.
Able to work as `no_std`.

Minimum supported rust compiler version (MSRV): 1.66.0; rust edition 2021.

## Main features

### Generics

Library proposes **generic implementation strategies**, which allow multiple
generic trait implementations.

Implementing trait for a generic type ("blanket implementation") more than once 
(applies both for local and foreign traits) - or implement foreign trait for a 
concrete type where there is some blanket implementation in the upstream. The 
solution is to use special pattern by @Kixunil. I use it widely and have a 
special helper type in [`src/strategy.rs`]()src/strategy.rs module.

With that helper type you can write the following code, which will provide you
with efficiently multiple blanket implementations of some trait `SampleTrait`:

```rust
pub trait SampleTrait {
    fn sample_trait_method(&self);
}

// Define strategies, one per specific implementation that you need,
// either blanket or concrete
pub struct StrategyA;
pub struct StrategyB;
pub struct StrategyC;

// Define a single marker type
pub trait Strategy {
    type Strategy;
}

// Do a single blanket implementation using Holder and Strategy marker trait
impl<T> SampleTrait for T
where
    T: Strategy + Clone,
    amplify::Holder<T, <T as Strategy>::Strategy>: SampleTrait,
{
    // Do this for each of sample trait methods:
    fn sample_trait_method(&self) {
        amplify::Holder::new(self.clone()).sample_trait_method()
    }
}

// Do this type of implementation for each of the strategies
impl<T> SampleTrait for amplify::Holder<T, StrategyA>
where
    T: Strategy,
{
    fn sample_trait_method(&self) {
        /* ... write your implementation-specific code here */
    }
}

# pub struct ConcreteTypeA;
// Finally, apply specific implementation strategy to a concrete type
// (or do it in a blanket generic way) as a marker:
impl Strategy for ConcreteTypeA {
    type Strategy = StrategyA;
}
```

### Derive macros

- Display
- From
- Error
- Getters
- AsAny
- Wrapper

A sample of what can be done with the macros:
```rust
#[derive(From, Error, Display, Debug)]
#[display(doc_comments)]
pub enum Error {
    // You can specify multiple conversions with separate attributes
    #[from(::std::io::Error)]
    #[from(IoError)]
    /// Generic I/O error
    Io,

    #[from]
    // This produces error description referencing debug representation
    // of the internal error type
    /// Formatting error: {_0:}
    Format(::std::fmt::Error),

    #[from]
    /// Some complex error, here are details: {details}
    WithFields { details: ::std::str::Utf8Error },

    #[display(LowerHex)]
    MultipleFields {
        // ...and you can also covert error type
        #[from(IoErrorUnit)]
        // rest of parameters must implement `Default`
        io: IoError,

        #[display(ToHex::to_hex)]
        details: String,
    },
}
```

More information is given in `amplify_derive` crate [README](derive/README.md).

### Macros

- `none!` as an alias for `Default::default()` on collection types and types
  for which semantics makes it sensible to emphasize that the operation 
  initializes empty structure.
- `s!` for fast `&str` -> `String` conversions
- Collection-generating macros:
  - `map!` & `bmap!` for a rappid `HashMap` and `BTreeMap` creation
  - `set!` & `bset!` for a rappid `HashSet` and `BTreeSet` creation
  - `list!` for `LinkedList`

### Wapper type

Wrapper trait helps in creating wrapped rust *newtypes*, Wrapped types are used 
for allowing implemeting foreign traits to foreign types:
<https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html>

Trait defines convenient methods for accessing inner data, construct
and deconstruct newtype. It also serves as a marker trait for newtypes.

The trait works well with `#[derive(Wrapper)]` from `amplify_derive` crate


## Build

```shell script
cargo build --all
cargo test
```

As a reminder, minimum supported rust compiler version (MSRV) is 1.36.0, so it
can be build with either nightly, dev, stable or 1.36+ version of the rust 
compiler. Use `rustup` for getting the proper version, or add `+toolchain`
parameter to both `cargo build` and `cargo test` commands.

## Benchmark

```shell
RUSTFLAGS="--cfg bench" cargo bench
```
