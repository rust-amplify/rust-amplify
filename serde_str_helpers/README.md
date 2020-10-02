# serde string helpers

Rust crate containing helpers for using serde with strings.

## About

Currently there is only a helper for deserializing stringly values more
efficiently by avoiding allocation (and copying) in certain cases. New helpers
may appear in the future.

## `DeserStrHelper`

A helper for deserializing using `TryFrom` more efficiently.

When using `#[serde(try_from = "String"]` when deserializing a value that
doesn't need to hold the string (e.g. an integer value) `serde` would
allocate the string even if it doesn't have to. (Such as in the case of
non-escaped Json string.)
                                                                            
A naive idea is to use `std::borrow::Cow` to solve it. Sadly, the
implementation of Deserialize for Cow<'de, str> doesn't borrow the string,
so it still allocates needlessly. This helper solves the issue.
                                                                            
Our DeserStrHelper is written such that it borrows the `str` when possible,
avoiding the allocation. It may still need to allocate, for example if
string decoding (unescaping) has to be performed.

## MSRV

The official MSRV is 1.41.1 for now and may be lowered later.

## License

MIT
