// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[cfg(feature = "std")]
use std::string::String;
use core::any::Any;
#[cfg(feature = "alloc")]
use alloc::string::String;

/// Trait `AsAny` allows simple conversion of any type into a generic "thick"
/// pointer `&dyn Any` (see [`Any`]), that can be later converted
/// back to the original type with a graceful failing for all other conversions.
/// For simple conversions it is recommended to use `#[derive(AsAny)]` macro
/// from `amplify_derive` crate (see [`amplify_derive::AsAny`]).
///
/// # Example
///
/// ```
/// #[macro_use]
/// use amplify::AsAny;
///
/// #[derive(AsAny, Copy, Clone, PartialEq, Eq, Debug)]
/// struct Point {
///     pub x: u64,
///     pub y: u64,
/// }
///
/// #[derive(AsAny, PartialEq, Debug)]
/// struct Circle {
///     pub radius: f64,
///     pub center: Point,
/// }
///
/// let mut point = Point { x: 1, y: 2 };
/// let point_ptr = point.as_any();
///
/// let mut circle = Circle {
///     radius: 18.,
///     center: point,
/// };
/// let circle_ptr = circle.as_any();
///
/// assert_eq!(point_ptr.downcast_ref(), Some(&point));
/// assert_eq!(circle_ptr.downcast_ref(), Some(&circle));
/// assert_eq!(circle_ptr.downcast_ref::<Point>(), None);
///
/// let p = point_ptr.downcast_ref::<Point>().unwrap();
/// assert_eq!(p.x, 1)
/// ```
pub trait AsAny {
    /// Returns thick pointer of `&dyn Any` type, that can be later downcasted
    /// back to a reference of the original type.
    fn as_any(&self) -> &dyn Any;
}

impl AsAny for usize {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u8 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u16 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u32 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u64 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for u128 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i8 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i16 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i32 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i64 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl AsAny for i128 {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

#[cfg(any(test, feature = "std", feature = "alloc"))]
impl AsAny for String {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

#[cfg(test)]
mod test {
    use ::core::any::Any;

    use super::AsAny;

    #[test]
    fn test_as_any_correct() {
        assert_eq!(1usize.as_any().type_id(), (&5usize as &dyn Any).type_id());
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
