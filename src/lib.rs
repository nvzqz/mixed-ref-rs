use std::ops::Deref;
use std::borrow::Borrow;

/// A reference to either owned or borrowed data.
///
/// If the data is owned, it's recommended to provide the `'static` lifetime.
/// In some cases, there's no other option.
pub enum MixedRef<'a, T: ?Sized + 'a> {
    /// Owned, boxed data.
    Owned(Box<T>),
    /// Borrowed data.
    Borrowed(&'a T)
}

impl<'a, T: Default> Default for MixedRef<'a, T> {
    fn default() -> Self {
        MixedRef::Owned(Default::default())
    }
}

impl<'a, T: ?Sized> From<&'a T> for MixedRef<'a, T> {
    fn from(r: &'a T) -> Self {
        MixedRef::Borrowed(r)
    }
}

impl<'a, T: ?Sized> From<Box<T>> for MixedRef<'a, T> {
    fn from(b: Box<T>) -> Self {
        MixedRef::Owned(b)
    }
}

impl<'a, T: ?Sized> Deref for MixedRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match *self {
            MixedRef::Owned(ref b) => b,
            MixedRef::Borrowed(r) => r
        }
    }
}

impl<'a, T: ?Sized> AsRef<T> for MixedRef<'a, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized> Borrow<T> for MixedRef<'a, T> {
    fn borrow(&self) -> &T {
        self
    }
}
