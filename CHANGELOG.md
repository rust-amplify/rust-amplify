Change Log
==========

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
