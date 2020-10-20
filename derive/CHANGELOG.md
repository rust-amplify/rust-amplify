Change Log
==========

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
