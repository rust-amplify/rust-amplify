Change Log
==========

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
