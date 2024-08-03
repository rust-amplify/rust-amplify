// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2022-2024 by
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

//! Confinement puts a constraint on the number of elements within a collection.

use core::borrow::{Borrow, BorrowMut};
use core::fmt::{self, Display, Formatter, LowerHex, UpperHex};
use core::str::FromStr;
use core::hash::Hash;
use core::ops::{
    Deref, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use alloc::vec::Vec;
use alloc::string::String;
use alloc::borrow::ToOwned;
use alloc::collections::{btree_map, BTreeMap, BTreeSet, VecDeque};
use core::slice::SliceIndex;
#[cfg(feature = "std")]
use std::{
    io, usize,
    collections::{hash_map, HashMap, HashSet},
};
use amplify_num::hex;
use amplify_num::hex::{FromHex, ToHex};
use ascii::{AsAsciiStrError, AsciiChar, AsciiString};

use crate::num::u24;

/// Trait implemented by a collection types which need to support collection
/// confinement.
pub trait Collection: FromIterator<Self::Item> + Extend<Self::Item> {
    /// Item type contained within the collection.
    type Item;

    /// Creates new collection with certain capacity.
    fn with_capacity(capacity: usize) -> Self;

    /// Returns the length of a collection.
    fn len(&self) -> usize;

    /// Detects whether collection is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes or inserts an element to the collection.
    fn push(&mut self, elem: Self::Item);

    /// Removes all elements from the collection.
    fn clear(&mut self);
}

/// Trait implemented by key-value maps which need to support collection
/// confinement.
pub trait KeyedCollection: Collection<Item = (Self::Key, Self::Value)> {
    /// Key type for the collection.
    type Key: Eq + Hash;
    /// Value type for the collection.
    type Value;
    type Entry<'a>
    where
        Self: 'a;

    /// Checks whether a given key is contained in the map.
    fn contains_key(&self, key: &Self::Key) -> bool;

    /// Gets mutable element of the collection.
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Returns iterator over keys and mutable values.
    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)>;

    /// Constructs iterator over mutable values.
    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value>;

    /// Inserts a new value under a key. Returns previous value if a value under
    /// the key was already present in the collection.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Removes a value stored under a given key, returning the owned value, if
    /// it was in the collection.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_>;
}

// Impls for main collection types

impl Collection for String {
    type Item = char;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl Collection for AsciiString {
    type Item = AsciiChar;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Collection for VecDeque<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.push_back(elem)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

#[cfg(feature = "std")]
impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.insert(elem);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    #[doc(hidden)]
    fn with_capacity(_capacity: usize) -> Self {
        BTreeSet::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        self.insert(elem);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        HashMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> KeyedCollection for HashMap<K, V> {
    type Key = K;
    type Value = V;
    type Entry<'a> = hash_map::Entry<'a, K, V> where K:'a, V: 'a;

    fn contains_key(&self, key: &Self::Key) -> bool {
        HashMap::contains_key(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        HashMap::get_mut(self, key)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)> {
        HashMap::iter_mut(self)
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value> {
        HashMap::values_mut(self)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        HashMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        HashMap::remove(self, key)
    }

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        HashMap::entry(self, key)
    }
}

impl<K: Ord + Hash, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    #[doc(hidden)]
    fn with_capacity(_capacity: usize) -> Self {
        BTreeMap::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn push(&mut self, elem: Self::Item) {
        BTreeMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Ord + Hash, V> KeyedCollection for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
    type Entry<'a> = btree_map::Entry<'a, K, V> where K: 'a, V: 'a;

    fn contains_key(&self, key: &Self::Key) -> bool {
        BTreeMap::contains_key(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        BTreeMap::get_mut(self, key)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)> {
        BTreeMap::iter_mut(self)
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value> {
        BTreeMap::values_mut(self)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        BTreeMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        BTreeMap::remove(self, key)
    }

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        BTreeMap::entry(self, key)
    }
}

// Errors

/// Errors when confinement constraints were not met.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// Operation results in collection reduced below the required minimum
    /// number of elements.
    Undersize {
        /** Current collection length */
        len: usize,
        /** Minimum number of elements which must be present in the
         * collection */
        min_len: usize,
    },

    /// Operation results in collection growth above the required maximum number
    /// of elements.
    Oversize {
        /** Current collection length */
        len: usize,
        /** Maximum number of elements which must be present in the
         * collection */
        max_len: usize,
    },

    /// Attempt to address an index outside the collection bounds.
    OutOfBoundary {
        /** Index which was outside the bounds */
        index: usize,
        /** The actual number of elements in the collection */
        len: usize,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Undersize { len, min_len } => write!(
                f,
                "operation results in collection size {len} less than lower boundary \
                 of {min_len}, which is prohibited"
            ),
            Error::Oversize { len, max_len } => write!(
                f,
                "operation results in collection size {len} exceeding {max_len}, \
                which is prohibited"
            ),
            Error::OutOfBoundary { index, len } => write!(
                f,
                "attempt to access the element at {index} which is outside of the \
                collection length boundary {len}"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Errors generated by constructing confined [`AsciiString`] from `str`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AsciiError {
    /// The string contains non-ASCII characters
    Ascii(AsAsciiStrError),

    /// Confinement requirements are violated
    Confinement(Error),
}

impl From<AsAsciiStrError> for AsciiError {
    fn from(err: AsAsciiStrError) -> Self {
        AsciiError::Ascii(err)
    }
}

impl From<Error> for AsciiError {
    fn from(err: Error) -> Self {
        AsciiError::Confinement(err)
    }
}

impl Display for AsciiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AsciiError::Ascii(e) => Display::fmt(e, f),
            AsciiError::Confinement(e) => Display::fmt(e, f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AsciiError {}

// Confinement params

/// Constant for a minimal size of a confined collection.
pub const ZERO: usize = 0;
/// Constant for a minimal size of a confined collection.
pub const ONE: usize = 1;
/// Constant for a maximal size of a confined collection equal to [`u8::MAX`].
pub const U8: usize = u8::MAX as usize;
/// Constant for a maximal size of a confined collection equal to [`u16::MAX`].
pub const U16: usize = u16::MAX as usize;
/// Constant for a maximal size of a confined collection equal to [`u24::MAX`].
pub const U24: usize = 0xFFFFFFusize;
/// Constant for a maximal size of a confined collection equal to [`u32::MAX`].
pub const U32: usize = u32::MAX as usize;
/// Constant for a maximal size of a confined collection equal to [`u64::MAX`].
pub const U64: usize = u64::MAX as usize;

// Confined collection

/// The confinement for the collection.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Confined<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize>(C);

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Deref
    for Confined<C, MIN_LEN, MAX_LEN>
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C, const MIN_LEN: usize, const MAX_LEN: usize> AsRef<[C::Item]>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Collection + AsRef<[C::Item]>,
{
    fn as_ref(&self) -> &[C::Item] {
        self.0.as_ref()
    }
}

impl<C, const MIN_LEN: usize, const MAX_LEN: usize> AsMut<[C::Item]>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Collection + AsMut<[C::Item]>,
{
    fn as_mut(&mut self) -> &mut [C::Item] {
        self.0.as_mut()
    }
}

impl<C, const MIN_LEN: usize, const MAX_LEN: usize> Borrow<[C::Item]>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Collection + Borrow<[C::Item]>,
{
    fn borrow(&self) -> &[C::Item] {
        self.0.borrow()
    }
}

impl<C, const MIN_LEN: usize, const MAX_LEN: usize> BorrowMut<[C::Item]>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Collection + BorrowMut<[C::Item]>,
{
    fn borrow_mut(&mut self) -> &mut [C::Item] {
        self.0.borrow_mut()
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IntoIterator
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IntoIterator,
{
    type Item = <C as IntoIterator>::Item;
    type IntoIter = <C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'c, C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IntoIterator
    for &'c Confined<C, MIN_LEN, MAX_LEN>
where
    &'c C: IntoIterator,
{
    type Item = <&'c C as IntoIterator>::Item;
    type IntoIter = <&'c C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'c, C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IntoIterator
    for &'c mut Confined<C, MIN_LEN, MAX_LEN>
where
    &'c mut C: IntoIterator,
{
    type Item = <&'c mut C as IntoIterator>::Item;
    type IntoIter = <&'c mut C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'c, C, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN>
where
    C: Collection + 'c,
    &'c mut C: IntoIterator<Item = &'c mut <C as Collection>::Item>,
{
    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter_mut(&'c mut self) -> <&'c mut C as IntoIterator>::IntoIter {
        let coll = &mut self.0;
        coll.into_iter()
    }
}

impl<C, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN>
where
    C: KeyedCollection,
{
    /// Returns an iterator that allows modifying each value for each key.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut C::Value> {
        let coll = &mut self.0;
        coll.values_mut()
    }

    /// Returns an iterator that allows modifying each value for each key.
    pub fn keyed_values_mut(&mut self) -> impl Iterator<Item = (&C::Key, &mut C::Value)> {
        let coll = &mut self.0;
        coll.iter_mut()
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<usize>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<usize, Output = C::Item>,
{
    type Output = C::Item;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<usize>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<usize, Output = C::Item>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<Range<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<Range<usize>, Output = [C::Item]>,
{
    type Output = [C::Item];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<Range<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<Range<usize>, Output = [C::Item]>,
{
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<RangeTo<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<RangeTo<usize>, Output = [C::Item]>,
{
    type Output = [C::Item];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<RangeTo<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<RangeTo<usize>, Output = [C::Item]>,
{
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<RangeFrom<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<RangeFrom<usize>, Output = [C::Item]>,
{
    type Output = [C::Item];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<RangeFrom<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<RangeFrom<usize>, Output = [C::Item]>,
{
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<RangeInclusive<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<RangeInclusive<usize>, Output = [C::Item]>,
{
    type Output = [C::Item];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<RangeInclusive<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<RangeInclusive<usize>, Output = [C::Item]>,
{
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<RangeToInclusive<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<RangeToInclusive<usize>, Output = [C::Item]>,
{
    type Output = [C::Item];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<RangeToInclusive<usize>>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<RangeToInclusive<usize>, Output = [C::Item]>,
{
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Index<RangeFull>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Index<RangeFull, Output = [C::Item]>,
{
    type Output = [C::Item];

    fn index(&self, index: RangeFull) -> &Self::Output {
        self.0.index(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> IndexMut<RangeFull>
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: IndexMut<RangeFull, Output = [C::Item]>,
{
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Display
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> FromStr
    for Confined<C, MIN_LEN, MAX_LEN>
where
    C: FromStr,
{
    type Err = C::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        C::from_str(s).map(Self)
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
    /// Constructs confinement over collection which was already size-checked.
    ///
    /// # Safety
    ///
    /// Panics if the collection size doesn't fit confinement type requirements.
    pub fn from_collection_unchecked(col: C) -> Self {
        Self::try_from(col).expect("collection size mismatch, use try_from instead")
    }

    #[deprecated(since = "4.7.0", note = "use from_collection_unchecked")]
    pub fn from_collection_unsafe(col: C) -> Self {
        Self::from_collection_unchecked(col)
    }

    /// Tries to construct a confinement over a collection. Fails if the number
    /// of items in the collection exceeds one of the confinement bounds.
    // We can't use `impl TryFrom` due to the conflict with core library blanked
    // implementation
    pub fn try_from(col: C) -> Result<Self, Error> {
        let len = col.len();
        if len < MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if len > MAX_LEN {
            return Err(Error::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(Self(col))
    }

    /// Tries to construct a confinement with a collection of elements taken
    /// from an iterator. Fails if the number of items in the collection
    /// exceeds one of the confinement bounds.
    pub fn try_from_iter<I: IntoIterator<Item = C::Item>>(iter: I) -> Result<Self, Error> {
        let mut col = C::with_capacity(MIN_LEN);
        for item in iter {
            col.push(item);
        }
        Self::try_from(col)
    }

    /// Construct a confinement with a collection of elements taken from an
    /// iterator. Panics if the number of items in the collection exceeds one
    /// of the confinement bounds.
    pub fn from_iter_unchecked<I: IntoIterator<Item = C::Item>>(iter: I) -> Self {
        Self::from_collection_unchecked(iter.into_iter().collect())
    }

    #[deprecated(since = "4.7.0", note = "use from_iter_unchecked")]
    pub fn from_iter_unsafe<I: IntoIterator<Item = C::Item>>(iter: I) -> Self {
        Self::from_iter_unchecked(iter)
    }

    /// Returns inner collection type
    #[deprecated(since = "4.7.0", note = "use as_unconfined method")]
    pub fn as_inner(&self) -> &C {
        &self.0
    }

    /// Returns reference to the inner collection type.
    pub fn as_unconfined(&self) -> &C {
        &self.0
    }

    /// Clones inner collection type and returns it
    #[deprecated(since = "4.7.0", note = "use to_unconfined method")]
    pub fn to_inner(&self) -> C
    where
        C: Clone,
    {
        self.0.clone()
    }

    /// Clones inner collection and returns an unconfined version of it.
    pub fn to_unconfined(&self) -> C
    where
        C: Clone,
    {
        self.0.clone()
    }

    /// Decomposes into the inner collection type
    #[deprecated(since = "4.7.0", note = "use release method")]
    pub fn into_inner(self) -> C {
        self.0
    }

    /// Attempts to add a single element to the confined collection. Fails if
    /// the number of elements in the collection already maximal.
    pub fn push(&mut self, elem: C::Item) -> Result<(), Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 > MAX_LEN {
            return Err(Error::Oversize {
                len: len + 1,
                max_len: MAX_LEN,
            });
        }
        self.0.push(elem);
        Ok(())
    }

    /// Attempts to add all elements from an iterator to the confined
    /// collection. Fails if the number of elements in the collection
    /// already maximal.
    pub fn extend<T: IntoIterator<Item = C::Item>>(&mut self, iter: T) -> Result<(), Error> {
        for elem in iter {
            self.push(elem)?;
        }
        Ok(())
    }

    /// Removes confinement and returns the underlying collection.
    #[deprecated(since = "4.7.0", note = "use release method")]
    pub fn unbox(self) -> C {
        self.0
    }

    /// Releases underlying collection from the confinement.
    pub fn release(self) -> C {
        self.0
    }
}

impl<C: Collection, const MAX_LEN: usize> Confined<C, ZERO, MAX_LEN>
where
    C: Default,
{
    /// Constructs a new confinement containing no elements.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new confinement containing no elements, but with a
    /// pre-allocated storage for the `capacity` of elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(C::with_capacity(capacity))
    }

    /// Removes all elements from the confined collection.
    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl<C: Collection, const MAX_LEN: usize> Default for Confined<C, ZERO, MAX_LEN>
where
    C: Default,
{
    fn default() -> Self {
        Self(C::default())
    }
}

impl<C: Collection, const MAX_LEN: usize> Confined<C, ONE, MAX_LEN>
where
    C: Default,
{
    /// Constructs a confinement with a collection made of a single required
    /// element.
    pub fn with(elem: C::Item) -> Self {
        let mut c = C::default();
        c.push(elem);
        Self(c)
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U8>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u8`. The
    /// confinement guarantees that the collection length can't exceed
    /// `u8::MAX`.
    pub fn len_u8(&self) -> u8 {
        self.len() as u8
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U16>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u16`. The
    /// confinement guarantees that the collection length can't exceed
    /// `u16::MAX`.
    pub fn len_u16(&self) -> u16 {
        self.len() as u16
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U24>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u24`. The
    /// confinement guarantees that the collection length can't exceed
    /// `u24::MAX`.
    pub fn len_u24(&self) -> u24 {
        u24::try_from(self.len() as u32).expect("confinement broken")
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U32>
where
    C: Default,
{
    /// Returns number of elements in the confined collection as `u32`. The
    /// confinement guarantees that the collection length can't exceed
    /// `u32::MAX`.
    pub fn len_u32(&self) -> u32 {
        self.len() as u32
    }
}

impl<C: KeyedCollection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
    /// Gets mutable reference to an element of the collection.
    pub fn get_mut(&mut self, key: &C::Key) -> Option<&mut C::Value> {
        self.0.get_mut(key)
    }

    /// Inserts a new value into the confined collection under a given key.
    /// Fails if the collection already contains maximum number of elements
    /// allowed by the confinement.
    pub fn insert(&mut self, key: C::Key, value: C::Value) -> Result<Option<C::Value>, Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 > MAX_LEN {
            return Err(Error::Oversize {
                len: len + 1,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.insert(key, value))
    }

    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation. Errors if the collection entry is vacant and the
    /// collection has already reached maximal size of its confinement.
    pub fn entry(&mut self, key: C::Key) -> Result<C::Entry<'_>, Error> {
        let len = self.len();
        if len == MAX_LEN && !self.0.contains_key(&key) {
            return Err(Error::Oversize {
                len: len + 1,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.entry(key))
    }
}

impl<C: KeyedCollection, const MAX_LEN: usize> Confined<C, ONE, MAX_LEN>
where
    C: Default,
{
    /// Constructs a confinement with a collection made of a single required
    /// key-value pair.
    pub fn with_key_value(key: C::Key, value: C::Value) -> Self {
        let mut c = C::default();
        c.insert(key, value);
        Self(c)
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> TryFrom<&str>
    for Confined<String, MIN_LEN, MAX_LEN>
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> TryFrom<&str>
    for Confined<AsciiString, MIN_LEN, MAX_LEN>
{
    type Error = AsciiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let a = AsciiString::from_str(value)?;
        Self::try_from(a).map_err(AsciiError::from)
    }
}

impl<const MAX_LEN: usize> Confined<String, ZERO, MAX_LEN> {
    /// Removes the last character from a string and returns it, or [`None`] if
    /// it is empty.
    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> Confined<String, MIN_LEN, MAX_LEN> {
    /// Removes a single character from the confined string, unless the string
    /// doesn't shorten more than the confinement requirement. Errors
    /// otherwise.
    pub fn remove(&mut self, index: usize) -> Result<char, Error> {
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index))
    }
}

impl<const MAX_LEN: usize> Confined<AsciiString, ZERO, MAX_LEN> {
    /// Removes the last character from a string and returns it, or [`None`] if
    /// it is empty.
    pub fn pop(&mut self) -> Option<AsciiChar> {
        self.0.pop()
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> Confined<AsciiString, MIN_LEN, MAX_LEN> {
    /// Removes a single character from the confined string, unless the string
    /// doesn't shorten more than the confinement requirement. Errors
    /// otherwise.
    pub fn remove(&mut self, index: usize) -> Result<AsciiChar, Error> {
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index))
    }
}

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<Vec<T>, MIN_LEN, MAX_LEN> {
    /// Constructs confinement out of slice of items. Does allocation.
    ///
    /// # Panics
    ///
    /// Panics if the size of the slice doesn't match the confinement type
    /// bounds.
    #[inline]
    pub fn from_slice_unchecked(slice: &[T]) -> Self
    where
        T: Clone,
    {
        assert!(slice.len() > MIN_LEN && slice.len() <= MAX_LEN);
        Self(slice.to_vec())
    }

    #[deprecated(since = "4.7.0", note = "use from_slice_unchecked")]
    #[inline]
    pub fn from_slice_unsafe(slice: &[T]) -> Self
    where
        T: Clone,
    {
        Self::from_slice_unchecked(slice)
    }

    /// Constructs confinement out of slice of items. Does allocation.
    #[inline]
    pub fn try_from_slice(slice: &[T]) -> Result<Self, Error>
    where
        T: Clone,
    {
        Self::try_from(slice.to_vec())
    }

    /// Returns slice representation of the vec.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Converts into the inner unconfined vector.
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Gets the mutable element of a vector
    #[inline]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.0.get_mut(index)
    }
}

impl<T, const MAX_LEN: usize> Confined<Vec<T>, ZERO, MAX_LEN> {
    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<Vec<T>, MIN_LEN, MAX_LEN> {
    /// Removes an element from the vector at a given index. Errors if the index
    /// exceeds the number of elements in the vector, of if the new vector
    /// length will be less than the confinement requirement. Returns the
    /// removed element otherwise.
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index))
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.0.iter()
    }
}

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<VecDeque<T>, MIN_LEN, MAX_LEN> {
    /// Removes the first element and returns it, or `None` if the deque is
    /// empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    /// Removes the last element and returns it, or `None` if the deque is
    /// empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<VecDeque<T>, MIN_LEN, MAX_LEN> {
    /// Prepends an element to the deque. Errors if the new collection length
    /// will not fit the confinement requirements.
    pub fn push_from(&mut self, elem: T) -> Result<(), Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 > MAX_LEN {
            return Err(Error::Oversize {
                len: len + 1,
                max_len: MAX_LEN,
            });
        }
        self.0.push_front(elem);
        Ok(())
    }

    /// Appends an element to the deque. Errors if the new collection length
    /// will not fit the confinement requirements.
    pub fn push_back(&mut self, elem: T) -> Result<(), Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 > MAX_LEN {
            return Err(Error::Oversize {
                len: len + 1,
                max_len: MAX_LEN,
            });
        }
        self.0.push_back(elem);
        Ok(())
    }

    /// Removes an element from the deque at a given index. Errors if the index
    /// exceeds the number of elements in the deque, of if the new deque
    /// length will be less than the confinement requirement. Returns the
    /// removed element otherwise.
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if index >= len {
            return Err(Error::OutOfBoundary { index, len });
        }
        Ok(self.0.remove(index).expect("element within the length"))
    }
}

#[cfg(feature = "std")]
impl<T: Hash + Eq, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<HashSet<T>, MIN_LEN, MAX_LEN>
{
    /// Removes an element from the set. Errors if the index exceeds the number
    /// of elements in the set, of if the new collection length will be less
    /// than the confinement requirement. Returns if the element was present
    /// in the set.
    pub fn remove(&mut self, elem: &T) -> Result<bool, Error> {
        if !self.0.contains(elem) {
            return Ok(false);
        }
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(elem))
    }

    /// Removes an element from the set. Errors if the index exceeds the number
    /// of elements in the set, of if the new collection length will be less
    /// than the confinement requirement. Returns the removed element
    /// otherwise.
    pub fn take(&mut self, elem: &T) -> Result<Option<T>, Error> {
        if !self.0.contains(elem) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.take(elem))
    }
}

impl<T: Ord, const MIN_LEN: usize, const MAX_LEN: usize> Confined<BTreeSet<T>, MIN_LEN, MAX_LEN> {
    /// Removes an element from the set. Errors if the index exceeds the number
    /// of elements in the set, of if the new collection length will be less
    /// than the confinement requirement. Returns if the element was present
    /// in the set.
    pub fn remove(&mut self, elem: &T) -> Result<bool, Error> {
        if !self.0.contains(elem) {
            return Ok(false);
        }
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(elem))
    }

    /// Removes an element from the set. Errors if the index exceeds the number
    /// of elements in the set, of if the new collection length will be less
    /// than the confinement requirement. Returns the removed element
    /// otherwise.
    pub fn take(&mut self, elem: &T) -> Result<Option<T>, Error> {
        if !self.0.contains(elem) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.take(elem))
    }
}

#[cfg(feature = "std")]
impl<K: Hash + Eq, V, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<HashMap<K, V>, MIN_LEN, MAX_LEN>
{
    /// Removes an element from the map. Errors if the index exceeds the number
    /// of elements in the map, of if the new collection length will be less
    /// than the confinement requirement. Returns the removed value
    /// otherwise.
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        if !self.0.contains_key(key) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(key))
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    pub fn into_keys(self) -> hash_map::IntoKeys<K, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `V`.
    pub fn into_values(self) -> hash_map::IntoValues<K, V> {
        self.0.into_values()
    }
}

impl<K: Ord + Hash, V, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    /// Removes an element from the map. Errors if the index exceeds the number
    /// of elements in the map, of if the new collection length will be less
    /// than the confinement requirement. Returns the removed value
    /// otherwise.
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        if !self.0.contains_key(key) {
            return Ok(None);
        }
        let len = self.len();
        if self.is_empty() || len <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(key))
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    pub fn into_keys(self) -> btree_map::IntoKeys<K, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `V`.
    pub fn into_values(self) -> btree_map::IntoValues<K, V> {
        self.0.into_values()
    }
}

// io::Writer
#[cfg(feature = "std")]
impl<const MAX_LEN: usize> io::Write for Confined<Vec<u8>, ZERO, MAX_LEN> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() + self.len() >= MAX_LEN {
            return Err(io::Error::from(io::ErrorKind::OutOfMemory));
        }
        self.0.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // Do nothing
        Ok(())
    }
}

// Vec<u8>-specific things

impl<const MIN_LEN: usize, const MAX_LEN: usize> LowerHex for Confined<Vec<u8>, MIN_LEN, MAX_LEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> UpperHex for Confined<Vec<u8>, MIN_LEN, MAX_LEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_hex().to_uppercase())
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> FromHex for Confined<Vec<u8>, MIN_LEN, MAX_LEN> {
    fn from_byte_iter<I>(iter: I) -> Result<Self, hex::Error>
    where
        I: Iterator<Item = Result<u8, hex::Error>> + ExactSizeIterator + DoubleEndedIterator,
    {
        Vec::<u8>::from_byte_iter(iter).map(Self)
    }
}

// Type aliases

/// [`String`] with maximum 255 characters.
pub type TinyString = Confined<String, ZERO, U8>;
/// [`String`] with maximum 2^16-1 characters.
pub type SmallString = Confined<String, ZERO, U16>;
/// [`String`] with maximum 2^24-1 characters.
pub type MediumString = Confined<String, ZERO, U24>;
/// [`String`] with maximum 2^32-1 characters.
pub type LargeString = Confined<String, ZERO, U32>;
/// Confined [`String`].
pub type ConfinedString<const MIN: usize = 0, const MAX: usize = U64> = Confined<String, MIN, MAX>;
/// [`String`] which contains at least a single character.
pub type NonEmptyString<const MAX: usize = U64> = Confined<String, ONE, MAX>;

/// [`AsciiString`] with maximum 255 characters.
pub type TinyAscii = Confined<AsciiString, ZERO, U8>;
/// [`AsciiString`] with maximum 2^16-1 characters.
pub type SmallAscii = Confined<AsciiString, ZERO, U16>;
/// [`AsciiString`] with maximum 2^24-1 characters.
pub type MediumAscii = Confined<AsciiString, ZERO, U24>;
/// [`AsciiString`] with maximum 2^32-1 characters.
pub type LargeAscii = Confined<AsciiString, ZERO, U32>;
/// Confined [`AsciiString`].
pub type ConfinedAscii<const MIN: usize = 0, const MAX: usize = U64> =
    Confined<AsciiString, MIN, MAX>;
/// [`AsciiString`] which contains at least a single character.
pub type NonEmptyAscii<const MAX: usize = U64> = Confined<AsciiString, ONE, MAX>;

/// [`Vec<u8>`] with maximum 255 characters.
pub type TinyBlob = Confined<Vec<u8>, ZERO, U8>;
/// [`Vec<u8>`] with maximum 2^16-1 characters.
pub type SmallBlob = Confined<Vec<u8>, ZERO, U16>;
/// [`Vec<u8>`] with maximum 2^24-1 characters.
pub type MediumBlob = Confined<Vec<u8>, ZERO, U24>;
/// [`Vec<u8>`] with maximum 2^32-1 characters.
pub type LargeBlob = Confined<Vec<u8>, ZERO, U32>;
/// Confined [`Vec<u8>`].
pub type ConfinedBlob<const MIN: usize = 0, const MAX: usize = U64> = Confined<Vec<u8>, MIN, MAX>;
/// [`Vec<u8>`] which contains at least a single character.
pub type NonEmptyBlob<const MAX: usize = U64> = Confined<Vec<u8>, ONE, MAX>;

/// [`Vec`] with maximum 255 items of type `T`.
pub type TinyVec<T> = Confined<Vec<T>, ZERO, U8>;
/// [`Vec`] with maximum 2^16-1 items of type `T`.
pub type SmallVec<T> = Confined<Vec<T>, ZERO, U16>;
/// [`Vec`] with maximum 2^24-1 items of type `T`.
pub type MediumVec<T> = Confined<Vec<T>, ZERO, U24>;
/// [`Vec`] with maximum 2^32-1 items of type `T`.
pub type LargeVec<T> = Confined<Vec<T>, ZERO, U32>;
/// Confined [`Vec`].
pub type ConfinedVec<T, const MIN: usize = 0, const MAX: usize = U64> = Confined<Vec<T>, MIN, MAX>;
/// [`Vec`] which contains at least a single item.
pub type NonEmptyVec<T, const MAX: usize = U64> = Confined<Vec<T>, ONE, MAX>;

/// [`VecDeque`] with maximum 255 items of type `T`.
pub type TinyDeque<T> = Confined<VecDeque<T>, ZERO, U8>;
/// [`VecDeque`] with maximum 2^16-1 items of type `T`.
pub type SmallDeque<T> = Confined<VecDeque<T>, ZERO, U16>;
/// [`VecDeque`] with maximum 2^24-1 items of type `T`.
pub type MediumDeque<T> = Confined<VecDeque<T>, ZERO, U24>;
/// [`VecDeque`] with maximum 2^32-1 items of type `T`.
pub type LargeDeque<T> = Confined<VecDeque<T>, ZERO, U32>;
/// Confined [`VecDeque`].
pub type ConfinedDeque<T, const MIN: usize = 0, const MAX: usize = U64> =
    Confined<VecDeque<T>, MIN, MAX>;
/// [`VecDeque`] which contains at least a single item.
pub type NonEmptyDeque<T, const MAX: usize = U64> = Confined<VecDeque<T>, ONE, MAX>;

/// [`HashSet`] with maximum 255 items of type `T`.
#[cfg(feature = "std")]
pub type TinyHashSet<T> = Confined<HashSet<T>, ZERO, U8>;
/// [`HashSet`] with maximum 2^16-1 items of type `T`.
#[cfg(feature = "std")]
pub type SmallHashSet<T> = Confined<HashSet<T>, ZERO, U16>;
/// [`HashSet`] with maximum 2^24-1 items of type `T`.
#[cfg(feature = "std")]
pub type MediumHashSet<T> = Confined<HashSet<T>, ZERO, U24>;
/// [`HashSet`] with maximum 2^32-1 items of type `T`.
#[cfg(feature = "std")]
pub type LargeHashSet<T> = Confined<HashSet<T>, ZERO, U32>;
#[cfg(feature = "std")]
/// Confined [`HashSet`].
pub type ConfinedHashSet<T, const MIN: usize = 0, const MAX: usize = U64> =
    Confined<HashSet<T>, MIN, MAX>;
/// [`HashSet`] which contains at least a single item.
#[cfg(feature = "std")]
pub type NonEmptyHashSet<T, const MAX: usize = U64> = Confined<HashSet<T>, ONE, MAX>;

/// [`BTreeSet`] with maximum 255 items of type `T`.
pub type TinyOrdSet<T> = Confined<BTreeSet<T>, ZERO, U8>;
/// [`BTreeSet`] with maximum 2^16-1 items of type `T`.
pub type SmallOrdSet<T> = Confined<BTreeSet<T>, ZERO, U16>;
/// [`BTreeSet`] with maximum 2^24-1 items of type `T`.
pub type MediumOrdSet<T> = Confined<BTreeSet<T>, ZERO, U24>;
/// [`BTreeSet`] with maximum 2^32-1 items of type `T`.
pub type LargeOrdSet<T> = Confined<BTreeSet<T>, ZERO, U32>;
/// Confined [`BTreeSet`].
pub type ConfinedOrdSet<T, const MIN: usize = 0, const MAX: usize = U64> =
    Confined<BTreeSet<T>, MIN, MAX>;
/// [`BTreeSet`] which contains at least a single item.
pub type NonEmptyOrdSet<T, const MAX: usize = U64> = Confined<BTreeSet<T>, ONE, MAX>;

/// [`HashMap`] with maximum 255 items.
#[cfg(feature = "std")]
pub type TinyHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U8>;
/// [`HashMap`] with maximum 2^16-1 items.
#[cfg(feature = "std")]
pub type SmallHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U16>;
/// [`HashMap`] with maximum 2^24-1 items.
#[cfg(feature = "std")]
pub type MediumHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U24>;
/// [`HashMap`] with maximum 2^32-1 items.
#[cfg(feature = "std")]
pub type LargeHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U32>;
#[cfg(feature = "std")]
/// Confined [`HashMap`].
pub type ConfinedHashMap<K, V, const MIN: usize = 0, const MAX: usize = U64> =
    Confined<HashSet<K, V>, MIN, MAX>;
/// [`HashMap`] which contains at least a single item.
#[cfg(feature = "std")]
pub type NonEmptyHashMap<K, V, const MAX: usize = U64> = Confined<HashMap<K, V>, ONE, MAX>;

/// [`BTreeMap`] with maximum 255 items.
pub type TinyOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U8>;
/// [`BTreeMap`] with maximum 2^16-1 items.
pub type SmallOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U16>;
/// [`BTreeMap`] with maximum 2^24-1 items.
pub type MediumOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U24>;
/// [`BTreeMap`] with maximum 2^32-1 items.
pub type LargeOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U32>;
/// Confined [`BTreeMap`].
pub type ConfinedOrdMap<K, V, const MIN: usize = 0, const MAX: usize = U64> =
    Confined<BTreeMap<K, V>, MIN, MAX>;
/// [`BTreeMap`] which contains at least a single item.
pub type NonEmptyOrdMap<K, V, const MAX: usize = U64> = Confined<BTreeMap<K, V>, ONE, MAX>;

/// Helper macro to construct confined string of a [`TinyString`] type
#[macro_export]
macro_rules! tiny_s {
    ($lit:literal) => {
        $crate::confinement::TinyString::try_from(s!($lit))
            .expect("static string for tiny_s literal cis too long")
    };
}

/// Helper macro to construct confined string of a [`SmallString`] type
#[macro_export]
macro_rules! small_s {
    ($lit:literal) => {
        $crate::confinement::SmallString::try_from(s!($lit))
            .expect("static string for small_s literal cis too long")
    };
}

/// Helper macro to construct confined string of a [`MediumString`] type
#[macro_export]
macro_rules! medium_s {
    ($lit:literal) => {
        $crate::confinement::MediumString::try_from(s!($lit))
            .expect("static string for medium_s literal cis too long")
    };
}

/// Helper macro to construct confined vector of a given type
#[macro_export]
macro_rules! confined_vec {
    ($elem:expr; $n:expr) => (
        $crate::confinement::Confined::try_from(vec![$elem; $n])
            .expect("inline confined_vec literal contains invalid number of items")
    );
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::Confined::try_from(vec![$($x,)+])
            .expect("inline confined_vec literal contains invalid number of items")
            .into()
    )
}

/// Helper macro to construct confined vector of a [`TinyVec`] type
#[macro_export]
macro_rules! tiny_vec {
    ($elem:expr; $n:expr) => (
        $crate::confinement::TinyVec::try_from(vec![$elem; $n])
            .expect("inline tiny_vec literal contains invalid number of items")
    );
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::TinyVec::try_from(vec![$($x,)+])
            .expect("inline tiny_vec literal contains invalid number of items")
    )
}

/// Helper macro to construct confined vector of a [`SmallVec`] type
#[macro_export]
macro_rules! small_vec {
    ($elem:expr; $n:expr) => (
        $crate::confinement::SmallVec::try_from(vec![$elem; $n])
            .expect("inline small_vec literal contains invalid number of items")
    );
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::SmallVec::try_from(vec![$($x,)+])
            .expect("inline small_vec literal contains invalid number of items")
    )
}

/// Helper macro to construct confined vector of a [`MediumVec`] type
#[macro_export]
macro_rules! medium_vec {
    ($elem:expr; $n:expr) => (
        $crate::confinement::MediumVec::try_from(vec![$elem; $n])
            .expect("inline medium_vec literal contains invalid number of items")
    );
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::MediumVec::try_from(vec![$($x,)+])
            .expect("inline medium_vec literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`HashSet`] of a given type
#[macro_export]
macro_rules! confined_set {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::Confined::try_from(set![$($x,)+])
            .expect("inline confined_set literal contains invalid number of items")
            .into()
    )
}

/// Helper macro to construct confined [`HashSet`] of a [`TinyHashSet`] type
#[macro_export]
macro_rules! tiny_set {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::TinyHashSet::try_from(set![$($x,)+])
            .expect("inline tiny_set literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`HashSet`] of a [`SmallHashSet`] type
#[macro_export]
macro_rules! small_set {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::SmallHashSet::try_from(set![$($x,)+])
            .expect("inline small_set literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`HashSet`] of a [`MediumHashSet`] type
#[macro_export]
macro_rules! medium_set {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::MediumHashSet::try_from(set![$($x,)+])
            .expect("inline medium_set literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`BTreeSet`] of a given type
#[macro_export]
macro_rules! confined_bset {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::Confined::try_from(bset![$($x,)+])
            .expect("inline confined_bset literal contains invalid number of items")
            .into()
    )
}

/// Helper macro to construct confined [`BTreeSet`] of a [`TinyOrdSet`] type
#[macro_export]
macro_rules! tiny_bset {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::TinyOrdSet::try_from(bset![$($x,)+])
            .expect("inline tiny_bset literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`BTreeSet`] of a [`SmallOrdSet`] type
#[macro_export]
macro_rules! small_bset {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::SmallOrdSet::try_from(bset![$($x,)+])
            .expect("inline small_bset literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`BTreeSet`] of a [`MediumOrdSet`] type
#[macro_export]
macro_rules! medium_bset {
    ($($x:expr),+ $(,)?) => (
        $crate::confinement::MediumOrdSet::try_from(bset![$($x,)+])
            .expect("inline medium_bset literal contains invalid number of items")
    )
}

/// Helper macro to construct confined [`HashMap`] of a given type
#[macro_export]
macro_rules! confined_map {
    ($($key:expr => $value:expr),+ $(,)?) => (
        $crate::confinement::Confined::try_from(map!{ $($key => $value),+ })
            .expect("inline confined_map literal contains invalid number of items")
            .into()
    )
}

/// Helper macro to construct confined [`HashMap`] of a [`TinyHashMap`] type
#[macro_export]
macro_rules! tiny_map {
    { $($key:expr => $value:expr),+ $(,)? } => {
        $crate::confinement::TinyHashMap::try_from(map!{ $($key => $value,)+ })
            .expect("inline tiny_map literal contains invalid number of items")
    }
}

/// Helper macro to construct confined [`HashMap`] of a [`SmallHashMap`] type
#[macro_export]
macro_rules! small_map {
    { $($key:expr => $value:expr),+ $(,)? } => {
        $crate::confinement::SmallHashMap::try_from(map!{ $($key => $value,)+ })
            .expect("inline small_map literal contains invalid number of items")
    }
}

/// Helper macro to construct confined [`HashMap`] of a [`MediumHashMap`] type
#[macro_export]
macro_rules! medium_map {
    { $($key:expr => $value:expr),+ $(,)? } => {
        $crate::confinement::MediumHashMap::try_from(map!{ $($key => $value,)+ })
            .expect("inline medium_map literal contains invalid number of items")
    }
}

/// Helper macro to construct confined [`BTreeMap`] of a given type
#[macro_export]
macro_rules! confined_bmap {
    ($($key:expr => $value:expr),+ $(,)?) => (
        $crate::confinement::Confined::try_from(bmap!{ $($key => $value),+ })
            .expect("inline confined_bmap literal contains invalid number of items")
            .into()
    )
}

/// Helper macro to construct confined [`BTreeMap`] of a [`TinyOrdMap`] type
#[macro_export]
macro_rules! tiny_bmap {
    { $($key:expr => $value:expr),+ $(,)? } => {
        $crate::confinement::TinyOrdMap::try_from(bmap!{ $($key => $value,)+ })
            .expect("inline tiny_bmap literal contains invalid number of items")
    }
}

/// Helper macro to construct confined [`BTreeMap`] of a [`SmallOrdMap`] type
#[macro_export]
macro_rules! small_bmap {
    { $($key:expr => $value:expr),+ $(,)? } => {
        $crate::confinement::SmallOrdMap::try_from(bmap!{ $($key => $value,)+ })
            .expect("inline small_bmap literal contains invalid number of items")
    }
}

/// Helper macro to construct confined [`BTreeMap`] of a [`MediumOrdMap`] type
#[macro_export]
macro_rules! medium_bmap {
    { $($key:expr => $value:expr),+ $(,)? } => {
        $crate::confinement::MediumOrdMap::try_from(bmap!{ $($key => $value,)+ })
            .expect("inline medium_bmap literal contains invalid number of items")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fits_max() {
        let mut s = TinyString::new();
        assert!(s.is_empty());
        for _ in 1..=255 {
            s.push('a').unwrap();
        }
        assert_eq!(s.len_u8(), u8::MAX);
        assert_eq!(s.len_u8(), s.len() as u8);
        assert!(!s.is_empty());

        let mut vec = TinyVec::new();
        let mut deque = TinyDeque::new();
        let mut set = TinyHashSet::new();
        let mut bset = TinyOrdSet::new();
        let mut map = TinyHashMap::new();
        let mut bmap = TinyOrdMap::new();
        assert!(vec.is_empty());
        assert!(deque.is_empty());
        assert!(set.is_empty());
        assert!(bset.is_empty());
        assert!(map.is_empty());
        assert!(bmap.is_empty());
        for index in 1..=255 {
            vec.push(5u8).unwrap();
            deque.push(5u8).unwrap();
            set.push(index).unwrap();
            bset.push(5u8).unwrap();
            map.insert(5u8, 'a').unwrap();
            bmap.insert(index, 'a').unwrap();
        }
        assert_eq!(vec.len_u8(), u8::MAX);
        assert_eq!(deque.len_u8(), u8::MAX);
        assert_eq!(set.len_u8(), u8::MAX);
        assert_eq!(bset.len_u8(), 1);
        assert_eq!(map.len_u8(), 1);
        assert_eq!(bmap.len_u8(), u8::MAX);

        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    #[should_panic(expected = "Oversize")]
    fn cant_go_above_max() {
        let mut s = TinyString::new();
        for _ in 1..=256 {
            s.push('a').unwrap();
        }
    }

    #[test]
    #[should_panic(expected = "Undersize")]
    fn cant_go_below_min() {
        let mut s = NonEmptyString::<U8>::with('a');
        s.remove(0).unwrap();
    }

    #[test]
    fn entry() {
        let mut col = tiny_bmap!(0u16 => 'a');
        assert!(matches!(
            col.entry(0).unwrap(),
            btree_map::Entry::Occupied(_)
        ));
        assert!(matches!(col.entry(1).unwrap(), btree_map::Entry::Vacant(_)));
        for idx in 1..u8::MAX {
            col.insert(idx as u16, 'b').unwrap();
        }
        assert!(matches!(
            col.entry(2).unwrap(),
            btree_map::Entry::Occupied(_)
        ));
        assert!(col.entry(256).is_err());
    }

    #[test]
    fn macros() {
        tiny_vec!("a", "b", "c");
        small_vec!("a", "b", "c");

        tiny_set!("a", "b", "c");
        tiny_bset!("a", "b", "c");
        tiny_map!("a" => 1, "b" => 2, "c" => 3);
        tiny_bmap!("a" => 1, "b" => 2, "c" => 3);

        small_set!("a", "b", "c");
        small_bset!("a", "b", "c");
        small_map!("a" => 1, "b" => 2, "c" => 3);
        small_bmap!("a" => 1, "b" => 2, "c" => 3);

        let _: TinyHashMap<_, _> = confined_map!("a" => 1, "b" => 2, "c" => 3);
    }

    #[test]
    fn iter_mut_btree() {
        let mut coll = tiny_bmap!(1 => "one");
        for (_index, item) in &mut coll {
            *item = "two";
        }
        assert_eq!(coll.get(&1), Some(&"two"));
        for (_index, item) in coll.keyed_values_mut() {
            *item = "three";
        }
        assert_eq!(coll.get(&1), Some(&"three"));
        for item in coll.values_mut() {
            *item = "four";
        }
        assert_eq!(coll.get(&1), Some(&"four"));
        *coll.get_mut(&1).unwrap() = "five";
        assert_eq!(coll.get(&1), Some(&"five"));
    }
}
