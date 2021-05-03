# Derive helper library

![Build](https://github.com/LNP-BP/rust-amplify/workflows/Build/badge.svg)
![Tests](https://github.com/LNP-BP/rust-amplify/workflows/Tests/badge.svg)
![Lints](https://github.com/LNP-BP/rust-amplify/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/LNP-BP/rust-amplify/branch/master/graph/badge.svg)](https://codecov.io/gh/LNP-BP/rust-amplify)

[![crates.io](https://meritbadge.herokuapp.com/amplify_syn)](https://crates.io/crates/amplify_syn)
[![Docs](https://docs.rs/amplify_syn/badge.svg)](https://docs.rs/amplify_syn)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Carefully crafted extensions to the well-known `syn` crate, which helps to
create complex derivation and proc macro libraries.

For samples, please check [documentation]((https://docs.rs/amplify_syn)) and 
the [following code](https://github.com/LNP-BP/rust-amplify/tree/master/derive/src/getters.rs) 
from `amplify_derive` crate, which uses this library for its custom derivation 
macros.

MSRV: 1.31 (required by `syn`)
