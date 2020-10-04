# Stringly conversions
[![crates.io](https://meritbadge.herokuapp.com/stringly_conversions)](https://crates.io/crates/stringly_conversions)
![Build](https://github.com/LNP-BP/rust-amplify/workflows/Build/badge.svg)
![Tests](https://github.com/LNP-BP/rust-amplify/workflows/Tests/badge.svg)
![Lints](https://github.com/LNP-BP/rust-amplify/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/LNP-BP/rust-amplify/branch/master/graph/badge.svg)](https://codecov.io/gh/LNP-BP/rust-amplify)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A crate helping to convert to/from various representations of strings.

## Features

* `no_std` with an optional feature to enable `alloc`
* Macros for implementing `TryFrom<Stringly> for YourType` where
  `YourType: FromStr`.
* Macros for implementing `From<YourType> for Stringly` using `Display`
