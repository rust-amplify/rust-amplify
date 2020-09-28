# Rust Amplify Library
![Rust](https://github.com/LNP-BP/rust-amplify/workflows/Rust/badge.svg)
[![crates.io](https://meritbadge.herokuapp.com/amplify)](https://crates.io/crates/amplify)
[![codecov](https://codecov.io/gh/LNP-BP/rust-amplify/branch/master/graph/badge.svg)](https://codecov.io/gh/LNP-BP/rust-amplify)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Amplifying Rust language capabilities: multiple generic trait implementations, 
type wrappers, derive macros.

## Main features

### Derive macros

- Display
- From
- Error
- Getters
- AsAny

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

See more in `amplify_derive` crate [README](derive/README.md).

### Macros

- `s!` for fast `&str` -> `String` conversions
- Collection-generating macros:
  - `map!` & `bmap!` for a rappid `HashMap` and `BTreeMap` creation
  - `set!` & `bset!` for a rappid `HashSet` and `BTreeSet` creation
  - `list!` for `LinkedList`

### Generics

Library proposes **generic implementation strategies**, which allow multiple
generic trait implementations. See `src/strategy.rs` mod for the details.

### Wapper type

TODO: write description

## Build

Important: for now this library uses rust nightly version, to unlock most
of Rust language power. This will change in the future with overall library
maturation.

```shell script
rustup install nightly
rustup default nightly
cargo build --all
cargo test
```

