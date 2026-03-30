// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2026 by Jinui agentic AI, supervised by
// Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use alloc::borrow::Cow;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::boxed::Box;
#[cfg(feature = "std")]
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "std")]
use std::cell::{Ref, RefMut};

/// A trait for types that can provide a reference to a [`VecDeque`].
pub trait AsDequeRef<T> {
    /// Returns a reference to the underlying [`VecDeque`].
    fn as_deque_ref(&self) -> &VecDeque<T>;
}

/// A trait for types that can provide a mutable reference to a [`VecDeque`].
pub trait AsDequeMut<T>: AsDequeRef<T> {
    /// Returns a mutable reference to the underlying [`VecDeque`].
    fn as_deque_mut(&mut self) -> &mut VecDeque<T>;
}

impl<T> AsDequeRef<T> for VecDeque<T> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self
    }
}

impl<T> AsDequeMut<T> for VecDeque<T> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        self
    }
}

impl<T, D> AsDequeRef<T> for &D
where
    D: AsDequeRef<T> + ?Sized,
{
    fn as_deque_ref(&self) -> &VecDeque<T> {
        (**self).as_deque_ref()
    }
}

impl<T, D> AsDequeRef<T> for &mut D
where
    D: AsDequeRef<T> + ?Sized,
{
    fn as_deque_ref(&self) -> &VecDeque<T> {
        (**self).as_deque_ref()
    }
}

impl<T, D> AsDequeMut<T> for &mut D
where
    D: AsDequeMut<T> + ?Sized,
{
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        (**self).as_deque_mut()
    }
}

impl<T> AsDequeRef<T> for Box<VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self.as_ref()
    }
}

impl<T> AsDequeMut<T> for Box<VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        self.as_mut()
    }
}

impl<T> AsDequeRef<T> for Rc<VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self.as_ref()
    }
}

impl<T: Clone> AsDequeMut<T> for Rc<VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        Rc::make_mut(self)
    }
}

impl<T> AsDequeRef<T> for Arc<VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self.as_ref()
    }
}

impl<T: Clone> AsDequeMut<T> for Arc<VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        Arc::make_mut(self)
    }
}

impl<T: Clone> AsDequeRef<T> for Cow<'_, VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self.as_ref()
    }
}

impl<T: Clone> AsDequeMut<T> for Cow<'_, VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        self.to_mut()
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeRef<T> for Ref<'_, VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeRef<T> for RefMut<'_, VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeMut<T> for RefMut<'_, VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeRef<T> for MutexGuard<'_, VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeMut<T> for MutexGuard<'_, VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeRef<T> for RwLockReadGuard<'_, VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeRef<T> for RwLockWriteGuard<'_, VecDeque<T>> {
    fn as_deque_ref(&self) -> &VecDeque<T> {
        self
    }
}

#[cfg(feature = "std")]
impl<T> AsDequeMut<T> for RwLockWriteGuard<'_, VecDeque<T>> {
    fn as_deque_mut(&mut self) -> &mut VecDeque<T> {
        self
    }
}
