Change Log
==========

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
