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

/// Shorthand for `DumbDefault::dumb_default()`
#[macro_export]
macro_rules! dumb {
    () => {
        DumbDefault::dumb_default()
    };
}

/// Macro for creating [`HashMap`] in the same manner as `vec!` is used for
/// [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let map = map! {
///     s!("key") => 5,
///     s!("other_key") => 10
/// };
/// # }
/// ```
#[macro_export]
macro_rules! map {
    { } =>  {
        {
            ::std::collections::HashMap::new()
        }
    };

    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
}

/// Macro for creating [`HashSet`] in the same manner as `vec!` is used for
/// [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let map = set![5, 6, 7];
/// # }
/// ```
///
/// NB: you can't use repeated values with [`HashSet`], unlike to [`Vec`]'s:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// assert_eq!(set![1, 2, 3, 1], set![1, 2, 3]);
/// # }
/// ```
#[macro_export]
macro_rules! set {
    { } =>  {
        {
            ::std::collections::HashSet::new()
        }
    };

    { $($value:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )+
            m
        }
    }
}

/// Macro for creating [`BTreeMap`] in the same manner as `vec!` is used for
/// [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let map = bmap! {
///     s!("key") => 5,
///     s!("other_key") => 10
/// };
/// # }
/// ```
#[macro_export]
macro_rules! bmap {
    { } =>  {
        {
            ::std::collections::BTreeMap::new()
        }
    };

    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
}

/// Macro for creating [`BTreeSet`] in the same manner as `vec!` is used for
/// [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let map = bset![5, 6, 7];
/// # }
/// ```
///
/// NB: you can't use repeated values with [`HashSet`], unlike to [`Vec`]'s:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// assert_eq!(bset![1, 2, 3, 1], bset![1, 2, 3]);
/// # }
/// ```
#[macro_export]
macro_rules! bset {
    { } =>  {
        {
            ::std::collections::BTreeSet::new()
        }
    };

    { $($value:expr),+ } => {
        {
            let mut m = ::std::collections::BTreeSet::new();
            $(
                m.insert($value);
            )+
            m
        }
    }
}

/// Macro for creating [`LinkedList`] in the same manner as `vec!` is used for
/// [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let list = list! {
///     s!("item one") =>
///     s!("item two") =>
///     s!("item three")
/// };
/// # }
/// ```
#[macro_export]
macro_rules! list {
    { } =>  {
        {
            ::std::collections::LinkedList::new()
        }
    };

    { $($value:expr)=>+ } => {
        {
            let mut m = ::std::collections::LinkedList::new();
            $(
                m.push_back($value);
            )+
            m
        }
    }
}
