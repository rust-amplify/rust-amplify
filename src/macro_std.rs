// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
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

/// Macro for creating [`std::collections::HashMap`] in the same manner as
/// `vec!` is used for [`Vec`]:
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

    { owned: $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
            m
        }
    };

    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
}

/// Macro for creating [`std::collections::HashSet`] in the same manner as
/// `vec!` is used for [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let map = set![5, 6, 7];
/// # }
/// ```
///
/// NB: you can't use repeated values with [`std::collections::HashSet`], unlike
/// to [`Vec`]'s:
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

    { owned: $($value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value.to_owned());
            )+
            m
        }
    };

    { $($value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )+
            m
        }
    }
}

/// Macro for creating [`std::collections::BTreeMap`] in the same manner as
/// `vec!` is used for [`Vec`]:
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

    { owned: $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
            m
        }
    };

    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
}

/// Macro for creating [`std::collections::BTreeSet`] in the same manner as
/// `vec!` is used for [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate amplify;
///
/// # fn main() {
/// let map = bset![5, 6, 7];
/// # }
/// ```
///
/// NB: you can't use repeated values with [`std::collections::HashSet`], unlike
/// to [`Vec`]'s:
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

    { owned: $($value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::BTreeSet::new();
            $(
                m.insert($value.to_owned());
            )+
            m
        }
    };

    { $($value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::BTreeSet::new();
            $(
                m.insert($value);
            )+
            m
        }
    }
}

/// Macro for creating [`std::collections::LinkedList`] in the same manner as
/// `vec!` is used for [`Vec`]:
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

    { owned: $($value:expr)=>+ } => {
        {
            let mut m = ::std::collections::LinkedList::new();
            $(
                m.push_back($value.to_owned());
            )+
            m
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
