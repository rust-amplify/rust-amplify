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
pub type Array16 = Array<16>;

/// Wrapper type for all slice-based 256-bit types implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
pub type Array32 = Array<32>;

/// Wrapper type for all slice-based 512-bit types implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
pub type Array64 = Array<64>;

/// Wrapper type for all fixed arrays implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<const LEN: usize>(
    #[cfg_attr(
        all(feature = "serde", feature = "hex"),
        serde(
            serialize_with = "serde_helpers::to_hex",
            deserialize_with = "serde_helpers::from_hex"
        )
    )]
    [u8; LEN],
);

impl<const LEN: usize> Array<LEN> {
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

    /// Constructs array filled with given value
    pub fn with(val: u8) -> Self {
        Self([val; LEN])
    }

    /// Constructs 256-bit array from a provided slice. If the slice length
    /// is not equal to 32 bytes, returns `None`
    pub fn from_slice(slice: impl AsRef<[u8]>) -> Option<Self> {
        let slice = slice.as_ref();
        if slice.len() != LEN {
            return None;
        }
        let mut inner = [0u8; LEN];
        inner.copy_from_slice(slice);
        Some(Self(inner))
    }

    /// Returns byte slice representation.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Returns mutable byte slice representation.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }

    /// Returns vector representing internal slice data
    #[allow(clippy::wrong_self_convention)]
    #[cfg(any(test, feature = "std", feature = "alloc"))]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl<const LEN: usize> Default for Array<LEN> {
    fn default() -> Self {
        let inner = [0u8; LEN];
        Self(inner)
    }
}

impl<const LEN: usize> AsRef<[u8]> for Array<LEN> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const LEN: usize> AsMut<[u8]> for Array<LEN> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl<const LEN: usize> Borrow<[u8]> for Array<LEN> {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl<const LEN: usize> BorrowMut<[u8]> for Array<LEN> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.0.borrow_mut()
    }
}

impl<const LEN: usize> Index<usize> for Array<LEN> {
    type Output = u8;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> Index<u8> for Array<LEN> {
    type Output = u8;
    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<const LEN: usize> Index<Range<usize>> for Array<LEN> {
    type Output = [u8];
    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> Index<RangeTo<usize>> for Array<LEN> {
    type Output = [u8];
    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> Index<RangeFrom<usize>> for Array<LEN> {
    type Output = [u8];
    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> Index<RangeInclusive<usize>> for Array<LEN> {
    type Output = [u8];
    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> Index<RangeToInclusive<usize>> for Array<LEN> {
    type Output = [u8];
    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> Index<RangeFull> for Array<LEN> {
    type Output = [u8];
    #[inline]
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> IndexMut<usize> for Array<LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const LEN: usize> IndexMut<u8> for Array<LEN> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T, const LEN: usize> From<T> for Array<LEN>
where
    T: Into<[u8; LEN]>,
{
    fn from(array: T) -> Self {
        Self(array.into())
    }
}

impl<const LEN: usize> Wrapper for Array<LEN> {
    type Inner = [u8; LEN];

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
impl<const LEN: usize> Display for Array<LEN> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self, f)
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> Debug for Array<LEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Array<{}>({})", LEN, self.to_hex())
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> FromStr for Array<LEN> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl<const LEN: usize> FromHex for Array<LEN> {
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
impl<const LEN: usize> LowerHex for Array<LEN> {
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
impl<const LEN: usize> UpperHex for Array<LEN> {
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

    use crate::hex::{FromHex, ToHex};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes `buffer` to a lowercase hex string.
    pub fn to_hex<S, const LEN: usize>(buffer: &[u8; LEN], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&buffer.as_ref().to_hex())
    }

    /// Deserializes a lowercase hex string to a `Vec<u8>`.
    pub fn from_hex<'de, D, const LEN: usize>(deserializer: D) -> Result<[u8; LEN], D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        String::deserialize(deserializer).and_then(|string| {
            let vec =
                Vec::<u8>::from_hex(&string).map_err(|_| D::Error::custom("wrong hex data"))?;
            if vec.len() != LEN {
                return Err(D::Error::custom("Wrong 32-byte slice data length"));
            }
            let mut slice32 = [0u8; LEN];
            slice32.copy_from_slice(&vec[..LEN]);
            Ok(slice32)
        })
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
        let slice32 = Array32::from_hex(s).unwrap();
        assert_eq!(Array32::from_str(s), Ok(slice32));

        assert_eq!(Array32::from_hex(&s.to_uppercase()), Ok(slice32));
        assert_eq!(
            Array32::from_str(&s[..30]),
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

        assert_eq!(Array32::from_slice(&data), Some(slice32));
        assert_eq!(Array32::from_slice(&data[..30]), None);
        assert_eq!(&slice32.to_vec(), &data);
        assert_eq!(&slice32.as_inner()[..], &data);
        assert_eq!(slice32.to_inner(), data);
        assert_eq!(slice32.into_inner(), data);
    }
}
