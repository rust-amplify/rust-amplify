// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2020-2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
use core::fmt::{self, Display, Debug, Formatter, LowerHex, UpperHex};
#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
use core::str::FromStr;
use core::ops::{Index, IndexMut, RangeFull};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
use crate::hex::{Error, FromHex, ToHex};
use crate::Wrapper;

/// Wrapper type for all slice-based 128-bit types implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
pub type Bytes16 = Array<u8, 16>;

/// Wrapper type for all slice-based 256-bit types implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
pub type Bytes32 = Array<u8, 32>;

/// Wrapper type for all slice-based 512-bit types implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
pub type Bytes64 = Array<u8, 64>;

/// Wrapper type for all fixed arrays implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T, const LEN: usize>([T; LEN]);

impl<T, const LEN: usize> Array<T, LEN> {
    /// Constructs array filled with given value
    pub fn with(val: T) -> Self
    where
        T: Copy,
    {
        Self([val; LEN])
    }

    /// Returns byte slice representation.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }

    /// Returns mutable byte slice representation.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }

    /// Returns vector representing internal slice data
    #[allow(clippy::wrong_self_convention)]
    #[cfg(any(test, feature = "std", feature = "alloc"))]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.0.to_vec()
    }
}

impl<const LEN: usize> Array<u8, LEN> {
    #[cfg(feature = "rand")]
    /// Generates array from `rand::thread_rng` random number generator
    pub fn random() -> Self {
        use rand::RngCore;
        let mut entropy = [0u8; LEN];
        rand::thread_rng().fill_bytes(&mut entropy);
        Array::from_inner(entropy)
    }

    /// Constructs array filled with zero bytes
    pub fn zero() -> Self {
        Self([0u8; LEN])
    }
}

impl<T, const LEN: usize> Array<T, LEN>
where
    T: Default + Copy,
{
    /// Constructs array made of default entries
    pub fn new() -> Self {
        Default::default()
    }

    /// Constructs 256-bit array from a provided slice. If the slice length
    /// is not equal to 32 bytes, returns `None`
    pub fn from_slice(slice: impl AsRef<[T]>) -> Option<Self> {
        let slice = slice.as_ref();
        if slice.len() != LEN {
            return None;
        }
        let mut inner = [T::default(); LEN];
        inner.copy_from_slice(slice);
        Some(Self(inner))
    }
}

impl<T, const LEN: usize> Default for Array<T, LEN>
where
    T: Default + Copy,
{
    fn default() -> Self {
        let inner = [T::default(); LEN];
        Self(inner)
    }
}

impl<T, const LEN: usize> AsRef<[T]> for Array<T, LEN> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T, const LEN: usize> AsMut<[T]> for Array<T, LEN> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T, const LEN: usize> Borrow<[T]> for Array<T, LEN> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T, const LEN: usize> BorrowMut<[T]> for Array<T, LEN> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T, const LEN: usize> Index<usize> for Array<T, LEN> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> Index<Range<usize>> for Array<T, LEN> {
    type Output = [T];
    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> Index<RangeTo<usize>> for Array<T, LEN> {
    type Output = [T];
    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> Index<RangeFrom<usize>> for Array<T, LEN> {
    type Output = [T];
    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> Index<RangeInclusive<usize>> for Array<T, LEN> {
    type Output = [T];
    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> Index<RangeToInclusive<usize>> for Array<T, LEN> {
    type Output = [T];
    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> Index<RangeFull> for Array<T, LEN> {
    type Output = [T];
    #[inline]
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<usize> for Array<T, LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<Range<usize>> for Array<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<RangeTo<usize>> for Array<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<RangeFrom<usize>> for Array<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<RangeInclusive<usize>> for Array<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<RangeToInclusive<usize>> for Array<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> IndexMut<RangeFull> for Array<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const LEN: usize> From<T> for Array<T, LEN>
where
    T: Into<[T; LEN]>,
{
    fn from(array: T) -> Self {
        Self(array.into())
    }
}

impl<T, const LEN: usize> Wrapper for Array<T, LEN> {
    type Inner = [T; LEN];

    #[inline]
    fn from_inner(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.0
    }

    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }

    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> Display for Array<u8, LEN> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self, f)
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> Debug for Array<u8, LEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Array<{}>({})", LEN, self.to_hex())
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> FromStr for Array<u8, LEN> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> FromHex for Array<u8, LEN> {
    fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
    where
        I: Iterator<Item = Result<u8, Error>> + ExactSizeIterator + DoubleEndedIterator,
    {
        let vec = Vec::<u8>::from_byte_iter(iter)?;
        if vec.len() != 32 {
            return Err(Error::InvalidLength(32, vec.len()));
        }
        let mut id = [0u8; LEN];
        id.copy_from_slice(&vec);
        Ok(Array(id))
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> LowerHex for Array<u8, LEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}..{}",
                self.0[..4].to_hex(),
                self.0[(self.0.len() - 4)..].to_hex()
            )
        } else {
            f.write_str(&self.0.to_hex())
        }
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> UpperHex for Array<u8, LEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}..{}",
                self.0[..4].to_hex().to_ascii_uppercase(),
                self.0[(self.0.len() - 4)..].to_hex().to_ascii_uppercase()
            )
        } else {
            f.write_str(&self.0.to_hex().to_ascii_uppercase())
        }
    }
}

#[cfg(all(feature = "serde", feature = "hex"))]
pub(crate) mod serde_helpers {
    //! Serde serialization helpers

    use serde::{Deserialize, Deserializer, Serializer, Serialize};

    use crate::Array;
    use crate::hex::{FromHex, ToHex};

    impl<const LEN: usize> Serialize for Array<u8, LEN> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.as_ref().to_hex())
        }
    }

    impl<'de, const LEN: usize> Deserialize<'de> for Array<u8, LEN> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::Error;
            String::deserialize(deserializer)
                .and_then(|string| {
                    let vec = Vec::<u8>::from_hex(&string)
                        .map_err(|_| D::Error::custom("wrong hex data"))?;
                    if vec.len() != LEN {
                        return Err(D::Error::custom("Wrong 32-byte slice data length"));
                    }
                    let mut slice32 = [0u8; LEN];
                    slice32.copy_from_slice(&vec[..LEN]);
                    Ok(slice32)
                })
                .map(Self)
        }
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::*;
    use crate::Wrapper;
    use crate::hex::FromHex;

    #[test]
    fn test_slice32_str() {
        let s = "a3401bcceb26201b55978ff705fecf7d8a0a03598ebeccf2a947030b91a0ff53";
        let slice32 = Bytes32::from_hex(s).unwrap();
        assert_eq!(Bytes32::from_str(s), Ok(slice32));

        assert_eq!(Bytes32::from_hex(&s.to_uppercase()), Ok(slice32));
        assert_eq!(
            Bytes32::from_str(&s[..30]),
            Err(Error::InvalidLength(32, 15))
        );

        assert_eq!(&slice32.to_string(), s);
        assert_eq!(format!("{:x}", slice32), s);
        assert_eq!(format!("{:X}", slice32), s.to_uppercase());
        assert_eq!(format!("{:?}", slice32), format!("Array<32>({})", s));
    }

    #[test]
    fn test_encoding() {
        let s = "a3401bcceb26201b55978ff705fecf7d8a0a03598ebeccf2a947030b91a0ff53";
        let slice32 = Array::from_hex(s).unwrap();

        let data = [
            0xa3, 0x40, 0x1b, 0xcc, 0xeb, 0x26, 0x20, 0x1b, 0x55, 0x97, 0x8f, 0xf7, 0x05, 0xfe,
            0xcf, 0x7d, 0x8a, 0x0a, 0x03, 0x59, 0x8e, 0xbe, 0xcc, 0xf2, 0xa9, 0x47, 0x03, 0x0b,
            0x91, 0xa0, 0xff, 0x53,
        ];

        assert_eq!(Bytes32::from_slice(&data), Some(slice32));
        assert_eq!(Bytes32::from_slice(&data[..30]), None);
        assert_eq!(&slice32.to_vec(), &data);
        assert_eq!(&slice32.as_inner()[..], &data);
        assert_eq!(slice32.to_inner(), data);
        assert_eq!(slice32.into_inner(), data);
    }
}
