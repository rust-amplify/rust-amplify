#![allow(dead_code, bare_trait_objects)]

#[macro_use]
extern crate amplify_derive;

#[derive(DisplayEnum)]
enum Options {
    Some,
    Other,
    None,
}

fn main() {}
