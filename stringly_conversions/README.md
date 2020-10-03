# Stringly conversions

A crate helping to convert to/from various representations of strings.

## Features

* `no_std` with an optional feature to enable `alloc`
* Macros for implementing `TryFrom<Stringly> for YourType` where
  `YourType: FromStr`.
* Macros for implementing `From<YourType> for Stringly` using `Display`
