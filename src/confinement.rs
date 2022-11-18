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

use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::Deref;
use std::usize;

use crate::num::u24;

pub trait Collection {
    type Item;

    fn with_capacity(capacity: usize) -> Self;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn add(&mut self, elem: Self::Item);

    fn remove_at(&mut self, index: usize) -> Self::Item;

    fn clear(&mut self);
}

pub trait KeyedCollection: Collection<Item = (Self::Key, Self::Value)> {
    type Key: Eq + Hash;
    type Value;

    fn inject(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    fn eject(&mut self, key: &Self::Key) -> Option<Self::Value>;
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        Vec::push(self, elem)
    }

    fn remove_at(&mut self, index: usize) -> Self::Item {
        Vec::remove(self, index)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn add(&mut self, elem: Self::Item) {
        HashMap::insert(self, elem.0, elem.1);
    }

    fn remove_at(&mut self, _index: usize) -> Self::Item {
        panic!("HashMap doesn't have a concept of an index")
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

// Errors

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display(inner)]
pub enum ConfinementError {
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
// impl FromIterator
// impl IntoIterator

impl<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize> Confined<C, MIN_LEN, MAX_LEN> {
    pub fn try_from(col: C) -> Result<Self, ConfinementError> {
        let len = col.len();
        if len < MIN_LEN {
            return Err(ConfinementError::Undersize {
                len,
                min_len: MIN_LEN,
            });
        }
        if len > MAX_LEN {
            return Err(ConfinementError::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(Self(col))
    }

    pub fn push(&mut self, elem: C::Item) -> Result<(), ConfinementError> {
        let len = self.len();
        if len >= MAX_LEN {
            return Err(ConfinementError::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.add(elem))
    }

    // TODO: Add remove_at, append, extend

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
        Self(C::with_capacity(capacity))
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
    pub fn insert(
        &mut self,
        key: C::Key,
        value: C::Value,
    ) -> Result<Option<C::Value>, ConfinementError> {
        let len = self.len();
        if len >= MAX_LEN {
            return Err(ConfinementError::Oversize {
                len,
                max_len: MAX_LEN,
            });
        }
        Ok(self.0.inject(key, value))
    }

    // TODO: Add remove, append, extend, clear
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

// Type aliases

pub type TinyVec<T> = Confined<Vec<T>, ZERO, U8>;
pub type SmallVec<T> = Confined<Vec<T>, ZERO, U16>;
pub type MediumVec<T> = Confined<Vec<T>, ZERO, U24>;
pub type LargeVec<T> = Confined<Vec<T>, ZERO, U32>;
pub type NonEmptyVec<T> = Confined<Vec<T>, ONE, USIZE>;
