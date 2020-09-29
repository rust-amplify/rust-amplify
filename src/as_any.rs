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

#[cfg(test)]
mod test {
    use super::AsAny;

    #[test]
    fn test_as_any_correct() {
        assert_eq!(1usize.as_any().downcast_ref::<usize>().unwrap(), &1usize);
        assert_eq!(1u8.as_any().downcast_ref::<u8>().unwrap(), &1u8);
        assert_eq!(1u16.as_any().downcast_ref::<u16>().unwrap(), &1u16);
        assert_eq!(1u32.as_any().downcast_ref::<u32>().unwrap(), &1u32);
        assert_eq!(1u64.as_any().downcast_ref::<u64>().unwrap(), &1u64);
        assert_eq!(1u128.as_any().downcast_ref::<u128>().unwrap(), &1u128);
        assert_eq!(1i8.as_any().downcast_ref::<i8>().unwrap(), &1i8);
        assert_eq!(1i16.as_any().downcast_ref::<i16>().unwrap(), &1i16);
        assert_eq!(1i32.as_any().downcast_ref::<i32>().unwrap(), &1i32);
        assert_eq!(1i64.as_any().downcast_ref::<i64>().unwrap(), &1i64);
        assert_eq!(1i128.as_any().downcast_ref::<i128>().unwrap(), &1i128);
        assert_eq!(
            s!("string").as_any().downcast_ref::<String>().unwrap(),
            &"string"
        );
    }

    #[test]
    fn test_as_any_wrong() {
        assert!(1usize.as_any().downcast_ref::<u32>().is_none());
        assert!(1i8.as_any().downcast_ref::<u8>().is_none());
        assert!(1i16.as_any().downcast_ref::<u16>().is_none());
        assert!(1i32.as_any().downcast_ref::<u32>().is_none());
        assert!(1i64.as_any().downcast_ref::<u64>().is_none());
        assert!(1i128.as_any().downcast_ref::<u128>().is_none());
        assert!(s!("str").as_any().downcast_ref::<&str>().is_none());
    }
}
