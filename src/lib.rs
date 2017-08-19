//! Mixed references to owned and borrowed memory.
//!
//! # Usage
//!
//! This crate is available [on crates.io](https://crates.io/crates/mixed_ref)
//! and can be used by adding the following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mixed_ref = "0.1.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! #[macro_use]
//! extern crate mixed_ref;
//! # fn main() {}
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(feature = "std")]
extern crate core;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::{String, Vec};

#[cfg(feature = "std")]
use std::borrow::Cow;

use core::ops::{Deref, DerefMut};
use core::borrow::{Borrow, BorrowMut};

/// A reference to either owned or borrowed data.
///
/// If the data is owned, it's recommended to provide the `'static` lifetime.
/// In some cases, there's no other option.
///
/// If mutably borrowing data, use [`MixedRefMut`].
///
/// [`MixedRefMut`]: enum.MixedRefMut.html
#[derive(Debug, Eq, Hash)]
pub enum MixedRef<'a, T: ?Sized + 'a> {
    /// Owned, boxed data.
    Owned(Box<T>),
    /// Borrowed data.
    Borrowed(&'a T)
}

/// A reference to either owned or mutably borrowed data.
///
/// This acts similarly to [`MixedRef`], except that the inner data is mutable.
///
/// [`MixedRef`]: enum.MixedRef.html
#[derive(Debug, Eq, Hash)]
pub enum MixedRefMut<'a, T: ?Sized + 'a> {
    /// Owned, boxed data.
    Owned(Box<T>),
    /// Borrowed, mutable data.
    Borrowed(&'a mut T)
}

impl<'a, T: ?Sized + AsRef<U>, U: ?Sized + PartialEq> PartialEq<T> for MixedRef<'a, U> {
    fn eq(&self, other: &T) -> bool {
        (self as &U) == other.as_ref()
    }
}

impl<'a, T: ?Sized + AsRef<U>, U: ?Sized + PartialEq> PartialEq<T> for MixedRefMut<'a, U> {
    fn eq(&self, other: &T) -> bool {
        (self as &U) == other.as_ref()
    }
}

impl<'a, T: Default> Default for MixedRef<'a, T> {
    fn default() -> Self {
        MixedRef::Owned(Default::default())
    }
}

impl<'a, T: Default> Default for MixedRefMut<'a, T> {
    fn default() -> Self {
        MixedRefMut::Owned(Default::default())
    }
}

impl<'a, T: ?Sized> From<&'a T> for MixedRef<'a, T> {
    fn from(r: &'a T) -> Self {
        MixedRef::Borrowed(r)
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for MixedRefMut<'a, T> {
    fn from(r: &'a mut T) -> Self {
        MixedRefMut::Borrowed(r)
    }
}

impl<'a, T: ?Sized> From<Box<T>> for MixedRef<'a, T> {
    fn from(b: Box<T>) -> Self {
        MixedRef::Owned(b)
    }
}

impl<'a, T: ?Sized> From<Box<T>> for MixedRefMut<'a, T> {
    fn from(b: Box<T>) -> Self {
        MixedRefMut::Owned(b)
    }
}

impl<'a> From<String> for MixedRef<'a, str> {
    fn from(s: String) -> Self {
        Self::from(s.into_boxed_str())
    }
}

impl<'a> From<String> for MixedRefMut<'a, str> {
    fn from(s: String) -> Self {
        Self::from(s.into_boxed_str())
    }
}

impl<'a, T> From<Vec<T>> for MixedRef<'a, [T]> {
    fn from(v: Vec<T>) -> Self {
        Self::from(v.into_boxed_slice())
    }
}

impl<'a, T> From<Vec<T>> for MixedRefMut<'a, [T]> {
    fn from(v: Vec<T>) -> Self {
        Self::from(v.into_boxed_slice())
    }
}

impl<'a, T: ?Sized> From<MixedRefMut<'a, T>> for MixedRef<'a, T> {
    fn from(r: MixedRefMut<'a, T>) -> Self {
        match r {
            MixedRefMut::Owned(b) => MixedRef::Owned(b),
            MixedRefMut::Borrowed(r) => MixedRef::Borrowed(r),
        }
    }
}

impl<'a, T: ?Sized> Deref for MixedRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match *self {
            MixedRef::Owned(ref b) => b,
            MixedRef::Borrowed(ref r) => r
        }
    }
}

impl<'a, T: ?Sized> Deref for MixedRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match *self {
            MixedRefMut::Owned(ref b) => b,
            MixedRefMut::Borrowed(ref r) => r
        }
    }
}

impl<'a, T: ?Sized> DerefMut for MixedRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match *self {
            MixedRefMut::Owned(ref mut b) => b,
            MixedRefMut::Borrowed(ref mut r) => r
        }
    }
}

impl<'a, T: ?Sized> AsRef<T> for MixedRef<'a, T> {
    fn as_ref(&self) -> &T { self }
}

impl<'a, T: ?Sized> AsRef<T> for MixedRefMut<'a, T> {
    fn as_ref(&self) -> &T { self }
}

impl<'a, T: ?Sized> AsMut<T> for MixedRefMut<'a, T> {
    fn as_mut(&mut self) -> &mut T { self }
}

impl<'a, T: ?Sized> Borrow<T> for MixedRef<'a, T> {
    fn borrow(&self) -> &T { self }
}

impl<'a, T: ?Sized> Borrow<T> for MixedRefMut<'a, T> {
    fn borrow(&self) -> &T { self }
}

impl<'a, T: ?Sized> BorrowMut<T> for MixedRefMut<'a, T> {
    fn borrow_mut(&mut self) -> &mut T { self }
}

impl<'a, T: ?Sized + ToOwned> From<Cow<'a, T>> for MixedRef<'a, T>
    where T::Owned: Into<Box<T>>
{
    fn from(cow: Cow<'a, T>) -> Self {
        match cow {
            Cow::Owned(b) => MixedRef::Owned(b.into()),
            Cow::Borrowed(r) => MixedRef::Borrowed(r),
        }
    }
}

impl<'a, T: ?Sized + ToOwned> Into<Cow<'a, T>> for MixedRef<'a, T>
    where Box<T>: Into<T::Owned>
{
    fn into(self) -> Cow<'a, T> {
        match self {
            MixedRef::Owned(b) => Cow::Owned(b.into()),
            MixedRef::Borrowed(r) => Cow::Borrowed(r),
        }
    }
}

impl<'a, T: ?Sized + ToOwned> Into<Cow<'a, T>> for MixedRefMut<'a, T>
    where Box<T>: Into<T::Owned>
{
    fn into(self) -> Cow<'a, T> { MixedRef::from(self).into() }
}

impl<'a, T: ?Sized> MixedRefMut<'a, T> {
    /// Downcasts `self` into a reference to immutable data.
    pub fn downcast(self) -> MixedRef<'a, T> {
        self.into()
    }
}
