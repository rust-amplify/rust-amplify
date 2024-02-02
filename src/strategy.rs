// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Martin Habovstiak <martin.habovstiak@gmail.com>
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

//! This is a trick for rust compiler helping to distinguish types implementing
//! mutually-exclusive traits (required until negative trait impls will be
//! there) Implemented after concept by Martin Habovštiak
//! <martin.habovstiak@gmail.com>
//!
//! The module proposes **generic implementation strategies**, which allow
//! multiple generic trait implementations.
//!
//! Implementing trait for a generic type ("blanket implementation") more than
//! once (applies both for local and foreign traits) - or implement foreign
//! trait for a concrete type where there is some blanket implementation in the
//! upstream. The solution is to use special pattern by @Kixunil. I use it
//! widely and have a special helper type in
//! [`src/strategy.rs`]()src/strategy.rs module.
//!
//! With that helper type you can write the following code, which will provide
//! you with efficiently multiple blanket implementations of some trait
//! `SampleTrait`:
//!
//! ```
//! pub trait SampleTrait {
//!     fn sample_trait_method(&self);
//! }
//!
//! // Define strategies, one per specific implementation that you need,
//! // either blanket or concrete
//! pub struct StrategyA;
//! pub struct StrategyB;
//! pub struct StrategyC;
//!
//! // Define a single marker type
//! pub trait Strategy {
//!     type Strategy;
//! }
//!
//! // Do a single blanket implementation using Holder and Strategy marker trait
//! impl<T> SampleTrait for T
//! where
//!     T: Strategy,
//!     for<'a> amplify::Holder<&'a T, T::Strategy>: SampleTrait,
//! {
//!     // Do this for each of sample trait methods:
//!     fn sample_trait_method(&self) {
//!         amplify::Holder::new(self).sample_trait_method()
//!     }
//! }
//!
//! // Do this type of implementation for each of the strategies
//! impl<'a, T> SampleTrait for amplify::Holder<&'a T, StrategyA>
//! where
//!     T: Strategy,
//! {
//!     fn sample_trait_method(&self) {
//!         /* write your implementation-specific code here accessing type data,
//!         when needed, via `self.as_inner()` */
//!     }
//! }
//!
//! # pub struct ConcreteTypeA;
//! // Finally, apply specific implementation strategy to a concrete type
//! // (or do it in a blanket generic way) as a marker:
//! impl Strategy for ConcreteTypeA {
//!     type Strategy = StrategyA;
//! }
//! ```

use ::core::marker::PhantomData;

/// Helper type allowing implementation of trait object for generic types
/// multiple times. In practice this type is never used
pub struct Holder<T, S>(T, PhantomData<S>);
impl<T, S> Holder<T, S> {
    /// Wraps type into a holder to apply necessary blanked implementations.
    #[inline]
    pub fn new(val: T) -> Self {
        Self(val, PhantomData)
    }

    /// Returns a reference to the wrapped type.
    #[inline]
    pub fn as_type(&self) -> &T {
        &self.0
    }
}
