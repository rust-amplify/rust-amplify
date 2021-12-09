Change Log
==========

2.10.0
------
- Improving `Getters`: replacing struct-level arguments in a smart way with
  field-level arguments
- `num::Error` implements `std::Error`

2.9.0
-----
- Fixing `Getters` behavior to match documentation: if none of getter options
  are provided a function with the same name as a field is generated returning
  reference to the field (previously due to a bug this function had `_ref` 
  suffix)

2.7.2
-----
- Using `core` instead of `std` in order to support no_std environment

2.7.1
-----
- Making display derivation macro not to produce clippy::if_same_then_else

2.4.4
-----
- Fixed display derivation with enums using inner representation specified at
  the enum (topmost) level

2.4.3
-----
- Fixed breaking change in `syn` violating semantic versioning

2.4.2
-----
- Fixing Wrapper::From automatic derive problem

2.4.1
-----
- Fix for missed Wrapper use in derive generated code

2.4.0
-----
- Wrapper supports wrapping Debug
- Wrapper supports wrapping usize-based Index and IndexMut
- Support for custom amplify crate naming in Wrapper derive macro
- Auto implementation `From<Wrapped> for Inner` in wrapper derive

2.3.1
-----
- Fixing display derive bug for enums using display with some external function

2.3.0
-----
- Using amplify 2.3.0
- Deriving `Wrapper` does not require `use amplify::Wrapper`

2.2.0
-----
- Support for {0}-style indexes in derive(Display)
- Auto From<T> for String implementation for Error derive
- Fixed problem with display(Debug) for enums


2.1.0
-----
- Multiple display derivation improvements
- Index wrapper derivations


2.0.7
-----
- Improvements to `#[display()]`: #26, #32
- Allows `#[display(inner)]` derive for enum variants with named fields
- Improvements to `#[derive(Display)]` struct representation: #30
- Fixed index wrapper derivations: #27

2.0.6
-----
- Itroduction of `#[wrapper()]` meta field for deriving from internal 
  representation
- More internal derive types: unitary operations, bitwise and rem.

2.0.5
-----
- Fixed rare case in Wrapper derive for types having synonymous 
  `add/mul/*_assign` methods

2.0.4
-----
- Fixing Wrapper derivation issue with multiple formatting traits ambiguity

2.0.3
-----
- No autoderive for Display in Wrapper (use `#[display(inner)]` to mimic the
  old behaviour)
- Fixing display tuple derive warning

2.0.2
-----
- Support for `inner` as a Display alias for `{_0}` variant

2.0.1
-----
- Fixing display derivation behaviour for typled structs

2.0.0
-----
- Support for alternative Display formatting with `alt` attribute parameter

1.2.0
-----
- Upgrading to `amplify` v1.2.0

1.1.0
-----
- Upgrading to `amplify` v1.1.0

1.0.0
-----
### New features
- New derive macros:
    * Display
    * From
    * Error
### Breaking changes
- Removed all utility functions (new derive macro use better and more
  efficient approach)
### CI & docs
- Tests moved into doc comments (previously were done with example builds)
- Better docs
- Library commits to Cargo.lock version
