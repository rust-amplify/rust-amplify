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

use ::core::any::Any;
//use ::std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

// TODO: We can't do a default implementation with current rust compiler
//       limitations, but we can do a derive macro for an automatic
//       implementation of the trait, which is trivial
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl AsAny for usize {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u8 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u16 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u32 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u64 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u128 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i8 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i16 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i32 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i64 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i128 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for String {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}
/*
impl<'a, T> AsAny for Vec<T> {
    fn as_any(&'a self) -> &'a dyn Any {
        self as &'a dyn Any
    }
}

impl<T> AsAny for HashSet<T> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl<T, U> AsAny for HashMap<T, U> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl<T> AsAny for BTreeSet<T> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl<T, U> AsAny for BTreeMap<T, U> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl<T> AsAny for VecDeque<T> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl<T, U> AsAny for (T, U) {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}
*/
