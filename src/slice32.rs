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

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
use crate::hex::{Error, FromHex, ToHex};
use crate::Wrapper;
use crate::wrapper::WrapperMut;

/// Wrapper type for all slice-based 256-bit types implementing many important
/// traits, so types based on it can simply derive their implementations.
///
/// Type keeps data in little-endian byte order and displays them in the same
/// order (like bitcoin SHA256 single hash type).
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Slice32(
    #[cfg_attr(
        all(feature = "serde", feature = "hex"),
        serde(
            serialize_with = "serde_helpers::to_hex",
            deserialize_with = "serde_helpers::from_hex"
        )
    )]
    [u8; 32],
);

impl Slice32 {
    #[cfg(feature = "rand")]
    /// Generates 256-bit array from `bitcoin::secp256k1::rand::thread_rng`
    /// random number generator
    pub fn random() -> Self {
        use rand::RngCore;
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut entropy);
        Slice32::from_inner(entropy)
    }

    /// Constructs 256-bit array from a provided slice. If the slice length
    /// is not equal to 32 bytes, returns `None`
    pub fn from_slice(slice: impl AsRef<[u8]>) -> Option<Slice32> {
        if slice.as_ref().len() != 32 {
            return None;
        }
        let mut inner = [0u8; 32];
        inner.copy_from_slice(slice.as_ref());
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

impl AsRef<[u8]> for Slice32 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for Slice32 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl Borrow<[u8]> for Slice32 {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl BorrowMut<[u8]> for Slice32 {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.0.borrow_mut()
    }
}

impl Index<usize> for Slice32 {
    type Output = u8;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<u8> for Slice32 {
    type Output = u8;
    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl Index<RangeFull> for Slice32 {
    type Output = [u8];
    #[inline]
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Slice32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IndexMut<u8> for Slice32 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T> From<T> for Slice32
where
    T: Into<[u8; 32]>,
{
    fn from(array: T) -> Self {
        Self(array.into())
    }
}

impl Wrapper for Slice32 {
    type Inner = [u8; 32];

    #[inline]
    fn from_inner(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.0
    }

    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

impl WrapperMut for Slice32 {
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl Display for Slice32 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self, f)
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl Debug for Slice32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Slice32({})", self.to_hex())
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl FromStr for Slice32 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl FromHex for Slice32 {
    fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
    where
        I: Iterator<Item = Result<u8, Error>> + ExactSizeIterator + DoubleEndedIterator,
    {
        let vec = Vec::<u8>::from_byte_iter(iter)?;
        if vec.len() != 32 {
            return Err(Error::InvalidLength(32, vec.len()));
        }
        let mut id = [0u8; 32];
        id.copy_from_slice(&vec);
        Ok(Slice32(id))
    }
}

#[cfg(all(feature = "hex", any(feature = "std", feature = "alloc")))]
impl LowerHex for Slice32 {
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
impl UpperHex for Slice32 {
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
    pub fn to_hex<S>(buffer: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&buffer.as_ref().to_hex())
    }

    /// Deserializes a lowercase hex string to a `Vec<u8>`.
    pub fn from_hex<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        String::deserialize(deserializer).and_then(|string| {
            let vec =
                Vec::<u8>::from_hex(&string).map_err(|_| D::Error::custom("wrong hex data"))?;
            if vec.len() != 32 {
                return Err(D::Error::custom("Wrong 32-byte slice data length"));
            }
            let mut slice32 = [0u8; 32];
            slice32.copy_from_slice(&vec[0..32]);
            Ok(slice32)
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Error, Slice32};
    use crate::Wrapper;
    use crate::hex::FromHex;
    use core::str::FromStr;

    #[test]
    fn test_slice32_str() {
        let s = "a3401bcceb26201b55978ff705fecf7d8a0a03598ebeccf2a947030b91a0ff53";
        let slice32 = Slice32::from_hex(s).unwrap();
        assert_eq!(Slice32::from_str(s), Ok(slice32));

        assert_eq!(Slice32::from_hex(&s.to_uppercase()), Ok(slice32));
        assert_eq!(
            Slice32::from_str(&s[..30]),
            Err(Error::InvalidLength(32, 15))
        );

        assert_eq!(&slice32.to_string(), s);
        assert_eq!(format!("{:x}", slice32), s);
        assert_eq!(format!("{:X}", slice32), s.to_uppercase());
        assert_eq!(format!("{:?}", slice32), format!("Slice32({})", s));
    }

    #[test]
    fn test_encoding() {
        let s = "a3401bcceb26201b55978ff705fecf7d8a0a03598ebeccf2a947030b91a0ff53";
        let slice32 = Slice32::from_hex(s).unwrap();

        let data = [
            0xa3, 0x40, 0x1b, 0xcc, 0xeb, 0x26, 0x20, 0x1b, 0x55, 0x97, 0x8f, 0xf7, 0x05, 0xfe,
            0xcf, 0x7d, 0x8a, 0x0a, 0x03, 0x59, 0x8e, 0xbe, 0xcc, 0xf2, 0xa9, 0x47, 0x03, 0x0b,
            0x91, 0xa0, 0xff, 0x53,
        ];

        assert_eq!(Slice32::from_slice(&data), Some(slice32));
        assert_eq!(Slice32::from_slice(&data[..30]), None);
        assert_eq!(&slice32.to_vec(), &data);
        assert_eq!(&slice32.as_inner()[..], &data);
        assert_eq!(slice32.to_inner(), data);
        assert_eq!(slice32.into_inner(), data);
    }
}
