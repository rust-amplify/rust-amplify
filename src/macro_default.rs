// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Martin Habovstiak <martin.habovstiak@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

/// Macro for quick & simple `&str` -> `String` conversion:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// enum Error {
///     Io(String),
/// }
///
/// impl From<std::io::Error> for Error {
///     fn from(err: std::io::Error) -> Error {
///         Self::Io(s!("I/O error"))
///     }
/// }
/// # }
/// ```
#[macro_export]
macro_rules! s {
    ( $str:literal ) => {
        String::from($str)
    };
}

/// This macro allows more semantically-clear code (which can be used especially
/// with structure initialization), indicating that instead of type value we are
/// generating no value at all (empty collection or data structure filled with
/// information indicating absence of data)
#[macro_export]
macro_rules! none {
    () => {
        Default::default()
    };
}

/// This macro allows more semantically-clear code (which can be used especially
/// with structure initialization), indicating that instead of type value we are
/// generating zero values (int types or byte slices filled with zeros)
#[macro_export]
macro_rules! zero {
    () => {
        Default::default()
    };
}

/// This macro allows more semantically-clear code (which can be used especially
/// with structure initialization), indicating that instead of type value we are
/// generating empty collection types
#[macro_export]
macro_rules! empty {
    () => {
        Default::default()
    };
}

/// Shorthand for `Default::default()`
#[macro_export]
macro_rules! default {
    () => {
        Default::default()
    };
}

/// Shorthand for `Dumb::dumb()`
#[macro_export]
macro_rules! dumb {
    () => {
        $crate::Dumb::dumb()
    };
}
