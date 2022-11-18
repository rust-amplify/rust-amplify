// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
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

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::Deref;
use std::usize;

use crate::num::u24;

pub trait Collection {
    type Item;

    fn with_cap(capacity: usize) -> Self;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn add(&mut self, elem: Self::Item);

    fn extend_from_iter<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Self::Item>;

    fn clear(&mut self);
}

pub trait KeyedCollection: Collection<Item = (Self::Key, Self::Value)> {
    type Key: Eq + Hash;
    type Value;

    fn inject(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    fn eject(&mut self, key: &Self::Key) -> Option<Self::Value>;
}

// Impls for main collection types

impl Collection for String {
    type Item = char;

    fn with_cap(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn with_cap(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        self.push(elem)
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Collection for VecDeque<T> {
    type Item = T;

    fn with_cap(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        self.push_back(elem)
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;

    fn with_cap(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        self.insert(elem);
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    #[doc(hidden)]
    fn with_cap(_capacity: usize) -> Self {
        BTreeSet::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        self.insert(elem);
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);

    fn with_cap(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        HashMap::insert(self, elem.0, elem.1);
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Eq + Hash, V> KeyedCollection for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn inject(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        HashMap::insert(self, key, value)
    }

    fn eject(&mut self, key: &Self::Key) -> Option<Self::Value> {
        HashMap::remove(self, key)
    }
}

impl<K: Ord + Hash, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    #[doc(hidden)]
    fn with_cap(_capacity: usize) -> Self {
        BTreeMap::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        BTreeMap::insert(self, elem.0, elem.1);
    }

    fn extend_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.extend(iter)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Ord + Hash, V> KeyedCollection for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    fn inject(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        BTreeMap::insert(self, key, value)
    }

    fn eject(&mut self, key: &Self::Key) -> Option<Self::Value> {
        BTreeMap::remove(self, key)
    }
}

// Errors

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display(inner)]
pub enum Error {
    #[display(
        "operation results in collection size {len} less than lower boundary \
         of {min_len}, which is prohibited"
    )]
    Undersize { len: usize, min_len: usize },

    #[display(
        "operation results in collection size {len} exceeding {max_len}, \
         which is prohibited"
    )]
    Oversize { len: usize, max_len: usize },

    #[display(
        "attempt to access the element at {index} which is outside of the collection length boundary {len}"
    )]
    OutOfBoundary { index: usize, len: usize },
}

// Confinement params

const ZERO: usize = 0;
const ONE: usize = 0;
const U8: usize = u8::MAX as usize;
const U16: usize = u16::MAX as usize;
const U24: usize = 1usize << 24;
const U32: usize = u32::MAX as usize;
const USIZE: usize = usize::MAX;

// Confined collection

#[derive(Clone, Hash, Debug)]
pub struct Confined<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize>(C);

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Deref
    for Confined<C, MIN_LEN, MAX_LEN>
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
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

    pub fn try_from_iter<I: IntoIterator<Item = C::Item>>(iter: I) -> Result<Self, Error> {
        let mut col = C::with_cap(MIN_LEN);
        for item in iter {
            col.add(item);
        }
        Self::try_from(col)
    }

    pub fn push(&mut self, elem: C::Item) -> Result<(), Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 >= MAX_LEN {
            return Err(Error::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.add(elem))
    }

    pub fn extend<T: IntoIterator<Item = C::Item>>(&mut self, iter: T) -> Result<(), Error> {
        for elem in iter {
            self.push(elem)?;
        }
        Ok(())
    }

    pub fn unbox(self) -> C {
        self.0
    }
}

impl<C: Collection, const MAX_LEN: usize> Confined<C, ZERO, MAX_LEN>
where
    C: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(C::with_cap(capacity))
    }

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
    pub fn with(elem: C::Item) -> Self {
        let mut c = C::default();
        c.add(elem);
        Self(c)
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U8>
where
    C: Default,
{
    pub fn len_u8(&self) -> u8 {
        self.len() as u8
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U16>
where
    C: Default,
{
    pub fn len_u16(&self) -> u16 {
        self.len() as u16
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U24>
where
    C: Default,
{
    pub fn len_u24(&self) -> u24 {
        u24::try_from(self.len() as u32).expect("confinement broken")
    }
}

impl<C: Collection, const MIN_LEN: usize> Confined<C, MIN_LEN, U32>
where
    C: Default,
{
    pub fn len_u32(&self) -> u32 {
        self.len() as u32
    }
}

impl<C: KeyedCollection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
    pub fn insert(&mut self, key: C::Key, value: C::Value) -> Result<Option<C::Value>, Error> {
        let len = self.len();
        if len == MAX_LEN || len + 1 >= MAX_LEN {
            return Err(Error::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.inject(key, value))
    }
}

impl<C: KeyedCollection, const MAX_LEN: usize> Confined<C, ONE, MAX_LEN>
where
    C: Default,
{
    pub fn with_key_value(key: C::Key, value: C::Value) -> Self {
        let mut c = C::default();
        c.inject(key, value);
        Self(c)
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> Confined<String, MIN_LEN, MAX_LEN> {
    pub fn remove(&mut self, index: usize) -> Result<char, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
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
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
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

impl<T, const MIN_LEN: usize, const MAX_LEN: usize> Confined<VecDeque<T>, MIN_LEN, MAX_LEN> {
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
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

impl<T: Hash + Eq, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<HashSet<T>, MIN_LEN, MAX_LEN>
{
    pub fn remove(&mut self, elem: &T) -> Result<bool, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(elem))
    }

    pub fn take(&mut self, elem: &T) -> Result<Option<T>, Error> {
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

impl<T: Ord, const MIN_LEN: usize, const MAX_LEN: usize> Confined<BTreeSet<T>, MIN_LEN, MAX_LEN> {
    pub fn remove(&mut self, elem: &T) -> Result<bool, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(elem))
    }

    pub fn take(&mut self, elem: &T) -> Result<Option<T>, Error> {
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

impl<K: Hash + Eq, V, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<HashMap<K, V>, MIN_LEN, MAX_LEN>
{
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(key))
    }
}

impl<K: Ord + Hash, V, const MIN_LEN: usize, const MAX_LEN: usize>
    Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        let len = self.len();
        if self.is_empty() || len - 1 <= MIN_LEN {
            return Err(Error::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        Ok(self.0.remove(key))
    }
}

// Type aliases

pub type TinyString = Confined<String, ZERO, U8>;
pub type SmallString = Confined<String, ZERO, U16>;
pub type MediumString = Confined<String, ZERO, U24>;
pub type LargeString = Confined<String, ZERO, U32>;
pub type NonEmptyString = Confined<String, ONE, USIZE>;

pub type TinyVec<T> = Confined<Vec<T>, ZERO, U8>;
pub type SmallVec<T> = Confined<Vec<T>, ZERO, U16>;
pub type MediumVec<T> = Confined<Vec<T>, ZERO, U24>;
pub type LargeVec<T> = Confined<Vec<T>, ZERO, U32>;
pub type NonEmptyVec<T> = Confined<Vec<T>, ONE, USIZE>;

pub type TinyDeque<T> = Confined<VecDeque<T>, ZERO, U8>;
pub type SmallDeque<T> = Confined<VecDeque<T>, ZERO, U16>;
pub type MediumDeque<T> = Confined<VecDeque<T>, ZERO, U24>;
pub type LargeDeque<T> = Confined<VecDeque<T>, ZERO, U32>;
pub type NonEmptyDeque<T> = Confined<VecDeque<T>, ONE, USIZE>;

pub type TinyHashSet<T> = Confined<HashSet<T>, ZERO, U8>;
pub type SmallHashSet<T> = Confined<HashSet<T>, ZERO, U16>;
pub type MediumHashSet<T> = Confined<HashSet<T>, ZERO, U24>;
pub type LargeHashSet<T> = Confined<HashSet<T>, ZERO, U32>;
pub type NonEmptyHashSet<T> = Confined<HashSet<T>, ONE, USIZE>;

pub type TinyOrdSet<T> = Confined<BTreeSet<T>, ZERO, U8>;
pub type SmallOrdSet<T> = Confined<BTreeSet<T>, ZERO, U16>;
pub type MediumOrdSet<T> = Confined<BTreeSet<T>, ZERO, U24>;
pub type LargeOrdSet<T> = Confined<BTreeSet<T>, ZERO, U32>;
pub type NonEmptyOrdSet<T> = Confined<BTreeSet<T>, ONE, USIZE>;

pub type TinyHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U8>;
pub type SmallHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U16>;
pub type MediumHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U24>;
pub type LargeHashMap<K, V> = Confined<HashMap<K, V>, ZERO, U32>;
pub type NonEmptyHashMap<K, V> = Confined<HashMap<K, V>, ONE, USIZE>;

pub type TinyOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U8>;
pub type SmallOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U16>;
pub type MediumOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U24>;
pub type LargeOrdMap<K, V> = Confined<BTreeMap<K, V>, ZERO, U32>;
pub type NonEmptyOrdMap<K, V> = Confined<BTreeMap<K, V>, ONE, USIZE>;
