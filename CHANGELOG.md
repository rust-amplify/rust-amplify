Change Log
==========

4.10.0
------

- New `CursorDeque` implementation in `io` module, extending over smart pointers wrapping `VecDeque`
- Updated CI: added support for ubuntu-24.04.arm runners
- General chore and clippy lint fixes
- confinement: add `IndexMap` support via `indexmap` feature

4.9.0
-----

- add method allowing safe confinement mutable access (`Confinement::as_mut_checked` and
  `Confinement::with_mut`)
- add `retain` and `try_retain` to a keyed collection based confinements
- deprecate `ByteArray` methods having `_unsafe` suffix
- add `MultiError` type for collecting multiple errors

4.8.0
-----

- fix `alloc`/`std` macro ambiguity

4.7.1
-----

- confinement: add `VecDeque` `truncate` and `drain` methods

4.7.0
-----

- maintenance release

4.6.2
-----

- MSRV bump to 1.75.0 due to use of `impl` in trait method returns
- confinement: rename `_unchecked` names with `_checked` suffix
- confinement: deprecate general macros
- confinement: add blob construction macros
- confinement: add `confined_s` macro
- confinement: add `Confined*` type aliases
- confinement: rename `_unsafe` to `_unchecked`
- confinement: add `values_mut` method to `KeyedCollections`
- confinement: support `entry` method for map confinements
- confinement: deprecate `*inner` methods and replace with better names
- confinement: remove `Wrapper` implementation

4.6.1
-----

- fix no-std build
- fix rust v1.80 compatibility

4.6.0
-----

- MSRV bump to 1.69.0
- traits: export raw traits
- remove `serde` JSON, YAML, TOML as MSRV breakers

4.5.1
-----

- confinement: add `Vec::get_mut` proxy

4.5.0
-----

- chore: update dependencies

4.4.0
-----

- macro: provide no-std versions
- macro: add collection constructions macro with automatic `to_owned`
- confinement: implement hex traits for `Vec<u8>`-based confinements

4.3.0
-----

- chore: bump MSRV and `amplify-derive` version
- confinement: add `Confined<Vec<T>>` convenience APIs (`from_slice_unsafe`, `try_from_slice`,
  `as_slice`, `into_vec`)
- confinement: add `Confined::from_iter_unsafe` constructor

4.2.0
-----

- array: add new `ByteArray` trait, deprecate `RawArray`
- array: add `ByteArray` methods to `Array` type
- array: add dedicated `FromSliceError` type
- array: deprecate `Array::from_slice`, add `Array::copy_from_slice`
- array: fix `Display` implementation
- confinement: add `Collection::from_collection_unsafe`

4.1.1
-----

- fix RawArray blanket implementation for Array types

4.1.0
-----

- add generic parameter to Array to support reverse string representation
- add Bytes, Bytes4, Bytes20 and Bytes32StrRev type aliases
- add support for no-std to confined collections

4.0.2
-----

- fix for Array serde encoding to support non-standard hex implementations
  on top
- update dependencies
- pin dependencies to maintain MSRV

4.0.1
-----

- fix FromHex Array implementation bug

3.14.0
------

- New collection confinement module

3.10.0
------

- `FlagVec::is_empty` and `count_flags` methods
- Use of v2.10 derivation crate

3.9.0
-----

- Bumped `derive` dependency version

3.8.2
-----

- Feature `hex` becomes default and independent from `std` in `amplify_num`
- `DivRem` trait implemented for all small integer types

3.8.1
-----

- `Index` and `IndexMut` traits implemented for `Slice32`

3.8.0
-----

- Better no_std support: `std` and `alloc` features for the main crate and
  `amplify_num`
- `amplify_num::he`x now works in no_std mode
- New Slice32 type
- Fixed MSRV broken by serdr_yaml dependency

3.7.0
-----

- Numerics moved into dedicated `amplify_num` crate
- Multiple fixes & improvements to numeric arithmetics
- Numeric API finalization

3.6.0
-----

- Bit-sized precise integers (`u5`, `u6`, `u7`, `u24`)
- Little-endian conversion functions to large numeric types
- Clippy code linting
- MSRV reduced to 1.36.0

3.5.0
-----

- Introducing large unsigned integer types based on `bitcoin` crate original
  code as `num` mod (`u256`, `u512`, `u1024`)
- Introducing `hex` mod with hexadecimal conversion traits and helpers from
  `bitcoin_hashes`
- Moving feature flag types from `descriptor-wallet` library
- Re-exporting `amplify_derive` derivation macros if `derive` feature is used
- Making `derive` feature default
- Improvements to `IoError` type (better `Debug` implementation)

3.4.0
-----

- Support for amplify_syn (re-exported as `proc_attr` if the same-named feature
  is used)
- Use of new `amplify_derive` version

3.1.0
-----

- Wrapper::copy()

3.0.0
-----

- Internet addresses moved to separate external `inet2_addr` crate in
  <https://github.com/internet2-org/rust-internet2>. This allows to get rid of
  complex dependencies (Tor, Ed25519) and vendored SSL support

2.4.0
-----

- Ordering for Internet types
- Efficient (clonable/copyable) representation of ::std::io::Error with IoError
  type

2.3.0
-----

- `DumbDefault` type
- New semantic macros (`default!` and `dumb_default!`)
- Serde helper traits for serialization into YAML, JSON and TOML in display
  derives

2.2.0
-----

- Updating aplify_derive dependency
- Implementation of Hash derive for internet address types

2.1.0
-----

- Internet address-specific error types (#31)

2.0.5
-----

- stringly_conversions are now separate feature

2.0.4
-----

- Including alloc feature from stringly_conversions mod

2.0.3
-----

- Fixing feature set related to stringly conversions and serde helpers

2.0.2
-----

- Fixing serde serialization helpers for `InetSocketAddr` and `InetSocketAddrExt`
  types

2.0.1
-----

- Fixing serialization for `InetSocketAddr` and `InetSocketAddrExt` types

2.0.0
-----

### New features

- Wrapper derive macro, replacing old declarative macto `wrapper!`, with support
  for generics and complex internal structure

### Breaking changes

- Adoption of the new `stringly_conversions` and `serde_str_helpers` crates.
  Crates are re-exported.
- Removal of `Service`, `TryService` and `Exec` traits, which are moved into new
  `lnpbp_service` crate
- Removal of `async` feature and trait (no needed once service traits got moved)
- New simple `none!()` macro for semantic representation of empty type creation
  with `Default::default()`

1.2.0
-----

### New features

- Exposing `vendored_openssl` feature introduced in the underlying `torut` crate
  that allows to build with vendored version of OpenSSL library (useful for
  mobile platforms)
- Inprovements to Internet addresses module

1.1.0
-----

### New features

- Transfer from LNP/BP Core Library:
    * `Service` & `TryService` traits
    * Internet & socket addresses supporting Tor
    * `Bipolar` trait for efficient stream management

### Breaking changes

- Refactored set of features

### CI & docs

- More advanced CI testing all features and dependency builds

1.0.0
-----

### New features

- Reworked derive library

### CI & docs

- Removed Travis CI, replaced with GitHub actions
- Code coverage testing with CodeCov
- Library commits to Cargo.lock version
