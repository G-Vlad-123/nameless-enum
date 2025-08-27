
#[derive(Clone, Copy)]
pub(crate) enum Never {}

macro_rules! impl_fmt_for_never {
    ( $trait:ident ) => {     
        impl core::fmt::$trait for Never {
            fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self { }
            }
        }
    };
    ( $($trait:ident)+ ) => {
        $(
            impl_fmt_for_never!{ $trait }
        )+
    }
}

impl_fmt_for_never!{Debug Display Binary Octal UpperHex LowerHex UpperExp LowerExp Pointer}

impl core::fmt::Write for Never {
    fn write_str(&mut self, _: &str) -> core::fmt::Result { Ok(()) }
    fn write_char(&mut self, _: char) -> core::fmt::Result { Ok(()) }
    fn write_fmt(&mut self, _: core::fmt::Arguments<'_>) -> core::fmt::Result { Ok(()) }
}

impl core::error::Error for Never {}

impl serde::Serialize for Never {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match *self {}
    }
}

impl<T: ?Sized> core::ops::RangeBounds<T> for Never {
    fn start_bound(&self) -> core::ops::Bound<&T> {
        match *self {}
    }

    fn end_bound(&self) -> core::ops::Bound<&T> {
        match *self {}
    }

    fn contains<U>(&self, _: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        match *self {}
    }
}

impl<Item> Extend<Item> for Never {
    fn extend<Iter: IntoIterator<Item = Item>>(&mut self, _: Iter) {
        match *self {}
    }
}

impl<T: ?Sized> AsRef<T> for Never {
    fn as_ref(&self) -> &T { match *self {} }
}

impl<T: ?Sized> AsMut<T> for Never {
    fn as_mut(&mut self) -> &mut T { match *self {} }
}

#[cfg(feature = "nightly_internals")]
impl<T: ?Sized + NotNever> core::borrow::Borrow<T> for Never {
    fn borrow(&self) -> &T { match *self {} }
}

#[cfg(feature = "nightly_internals")]
impl<T: ?Sized + NotNever> core::borrow::BorrowMut<T> for Never {
    fn borrow_mut(&mut self) -> &mut T { match *self {} }
}

#[cfg(feature = "nightly_internals")]
pub(crate) auto trait NotNever {}

#[cfg(feature = "nightly_internals")]
impl !NotNever for Never {}

#[cfg(feature = "nightly_internals")]
impl<A, B, C, D, E, F, G, H, I, J, K, L> NotNever for super::En<A, B, C, D, E, F, G, H, I, J, K, L> {}

