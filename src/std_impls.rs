
mod convert {
    use crate::*;

    impl From<core::convert::Infallible> for En {
        fn from(value: core::convert::Infallible) -> Self {
            match value {}
        }
    }

    impl From<En> for core::convert::Infallible {
        fn from(value: En) -> Self {
            match value {}
        }
    }

    impl<A, B, C, D, E, F, G, H, I, J, K, L> From<En<A, B, C, D, E, F, G, H, I, J, K, L>> for ExEn<A, B, C, D, E, F, G, H, I, J, K, L> {
        fn from(en: En<A, B, C, D, E, F, G, H, I, J, K, L>) -> Self {
            match en {
                En::A(value) => ExEn::A(value),
                En::B(value) => ExEn::B(value),
                En::C(value) => ExEn::C(value),
                En::D(value) => ExEn::D(value),
                En::E(value) => ExEn::E(value),
                En::F(value) => ExEn::F(value),
                En::G(value) => ExEn::G(value),
                En::H(value) => ExEn::H(value),
                En::I(value) => ExEn::I(value),
                En::J(value) => ExEn::J(value),
                En::K(value) => ExEn::K(value),
                En::L(value) => ExEn::L(value),
            }
        }
    }

    impl<A, B, C, D, E, F, G, H, I, J, K, L> From<ExEn<A, B, C, D, E, F, G, H, I, J, K, L>> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
        fn from(ex_en: ExEn<A, B, C, D, E, F, G, H, I, J, K, L>) -> Self {
            match ex_en {
                ExEn::A(value) => En::A(value),
                ExEn::B(value) => En::B(value),
                ExEn::C(value) => En::C(value),
                ExEn::D(value) => En::D(value),
                ExEn::E(value) => En::E(value),
                ExEn::F(value) => En::F(value),
                ExEn::G(value) => En::G(value),
                ExEn::H(value) => En::H(value),
                ExEn::I(value) => En::I(value),
                ExEn::J(value) => En::J(value),
                ExEn::K(value) => En::K(value),
                ExEn::L(value) => En::L(value),
            }
        }
    }
}

mod cmp {
    use crate::*;

    impl Eq for En {}

    impl PartialEq for En {
        #[inline(always)] fn eq(&self, _: &Self) -> bool { true }
        #[inline(always)] fn ne(&self, _: &Self) -> bool { false }
    }

    impl PartialOrd for En {
        #[inline(always)] fn partial_cmp(&self, _: &Self) -> Option<core::cmp::Ordering> {
            Some(core::cmp::Ordering::Equal)
        }

        #[inline(always)] fn gt(&self, _: &Self) -> bool { false }
        #[inline(always)] fn lt(&self, _: &Self) -> bool { false }
        #[inline(always)] fn ge(&self, _: &Self) -> bool { true }
        #[inline(always)] fn le(&self, _: &Self) -> bool { true }
    }

    impl Ord for En {
        #[inline(always)] fn cmp(&self, _: &Self) -> core::cmp::Ordering {
            core::cmp::Ordering::Equal
        }

        #[inline(always)] fn max(self, _: Self) -> Self { match self {} }
        #[inline(always)] fn min(self, _: Self) -> Self { match self {} }
        #[inline(always)] fn clamp(self, _: Self, _: Self) -> Self { match self {} }
    }

    impl<A: Ord> Ord for En<A> {
        #[inline] fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            let En::A(this) = self else {
                unreachable!()
            };

            let En::A(other) = other else {
                unreachable!()
            };

            this.cmp(other)
        }

        #[inline] fn max(self, other: Self) -> Self {
            let En::A(this) = self;
            let En::A(other) = other;

            En::A(this.max(other))
        }

        #[inline] fn min(self, other: Self) -> Self {
            let En::A(this) = self;
            let En::A(other) = other;

            En::A(this.min(other))
        }

        #[inline] fn clamp(self, min: Self, max: Self) -> Self {
            let En::A(this) = self;
            let En::A(min) = min;
            let En::A(max) = max;

            En::A(this.clamp(min, max))
        }
    }

    macro_rules! impl_cmp_for_en {
        ( $($ty:ident)+ ) => {
            impl<$($ty: Eq),+> Eq for En<$($ty),+> {}
            
            impl<$($ty: PartialEq),+> PartialEq for En<$($ty),+> {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left == right,
                        )+
                        (_, _) => false
                    }
                }

                #[inline]
                fn ne(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left == right,
                        )+
                        (_, _) => false
                    }
                }
            }
            
            impl<$($ty: PartialOrd),+> PartialOrd for En<$($ty),+> {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left.partial_cmp(right),
                        )+
                        (_, _) => None
                    }
                }

                #[inline]
                fn gt(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left.gt(right),
                        )+
                        (_, _) => false
                    }
                }

                #[inline]
                fn lt(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left.lt(right),
                        )+
                        (_, _) => false
                    }
                }

                #[inline]
                fn ge(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left.ge(right),
                        )+
                        (_, _) => false
                    }
                }

                #[inline]
                fn le(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (En::$ty(left), En::$ty(right)) => left.le(right),
                        )+
                        (_, _) => false
                    }
                }
            }
        };
    }

    impl_cmp_for_en!{A}
    impl_cmp_for_en!{A B}
    impl_cmp_for_en!{A B C}
    impl_cmp_for_en!{A B C D}
    impl_cmp_for_en!{A B C D E}
    impl_cmp_for_en!{A B C D E F}
    impl_cmp_for_en!{A B C D E F G}
    impl_cmp_for_en!{A B C D E F G H}
    impl_cmp_for_en!{A B C D E F G H I}
    impl_cmp_for_en!{A B C D E F G H I J}
    impl_cmp_for_en!{A B C D E F G H I J K}
    impl_cmp_for_en!{A B C D E F G H I J K L}
}

mod fmt {
    use crate::*;

    use core::fmt::{
        Debug,
        Display,
        Binary,
        Octal,
        LowerHex,
        UpperHex,
        LowerExp,
        UpperExp,
        Pointer,
        Formatter,
        Result as FmtResult,
        Write,
    };

    macro_rules! impl_fmt_for_en {
        ( $trait:ident ) => {     
            impl<
                A: $trait,
                B: $trait,
                C: $trait,
                D: $trait,
                E: $trait,
                F: $trait,
                G: $trait,
                H: $trait,
                I: $trait,
                J: $trait,
                K: $trait,
                L: $trait,
            > $trait for En<A, B, C, D, E, F, G, H, I, J, K, L> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult {
                    match self {
                        En::A(value) => value.fmt(f),
                        En::B(value) => value.fmt(f),
                        En::C(value) => value.fmt(f),
                        En::D(value) => value.fmt(f),
                        En::E(value) => value.fmt(f),
                        En::F(value) => value.fmt(f),
                        En::G(value) => value.fmt(f),
                        En::H(value) => value.fmt(f),
                        En::I(value) => value.fmt(f),
                        En::J(value) => value.fmt(f),
                        En::K(value) => value.fmt(f),
                        En::L(value) => value.fmt(f),
                    }
                }
            }
        };
        ( $($trait:ident)+ ) => {
            $(
                impl_fmt_for_en!{ $trait }
            )+
        }
    }

    impl_fmt_for_en!{Debug Display Binary Octal UpperHex LowerHex UpperExp LowerExp Pointer}

    impl<
        A: core::error::Error, 
        B: core::error::Error, 
        C: core::error::Error, 
        D: core::error::Error, 
        E: core::error::Error, 
        F: core::error::Error, 
        G: core::error::Error, 
        H: core::error::Error, 
        I: core::error::Error, 
        J: core::error::Error, 
        K: core::error::Error, 
        L: core::error::Error,
    > core::error::Error for En<A, B, C, D, E, F, G, H, I, J, K, L> {}

    impl<
        A: Write, 
        B: Write, 
        C: Write, 
        D: Write, 
        E: Write, 
        F: Write, 
        G: Write, 
        H: Write, 
        I: Write, 
        J: Write, 
        K: Write, 
        L: Write,
    > Write for En<A, B, C, D, E, F, G, H, I, J, K, L> {
        fn write_char(&mut self, c: char) -> FmtResult {
            match self {
                En::A(writee) => writee.write_char(c),
                En::B(writee) => writee.write_char(c),
                En::C(writee) => writee.write_char(c),
                En::D(writee) => writee.write_char(c),
                En::E(writee) => writee.write_char(c),
                En::F(writee) => writee.write_char(c),
                En::G(writee) => writee.write_char(c),
                En::H(writee) => writee.write_char(c),
                En::I(writee) => writee.write_char(c),
                En::J(writee) => writee.write_char(c),
                En::K(writee) => writee.write_char(c),
                En::L(writee) => writee.write_char(c),
            }
        }

        fn write_str(&mut self, s: &str) -> FmtResult {
            match self {
                En::A(writee) => writee.write_str(s),
                En::B(writee) => writee.write_str(s),
                En::C(writee) => writee.write_str(s),
                En::D(writee) => writee.write_str(s),
                En::E(writee) => writee.write_str(s),
                En::F(writee) => writee.write_str(s),
                En::G(writee) => writee.write_str(s),
                En::H(writee) => writee.write_str(s),
                En::I(writee) => writee.write_str(s),
                En::J(writee) => writee.write_str(s),
                En::K(writee) => writee.write_str(s),
                En::L(writee) => writee.write_str(s),
            }
        }

        fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> FmtResult {
            match self {
                En::A(writee) => writee.write_fmt(args),
                En::B(writee) => writee.write_fmt(args),
                En::C(writee) => writee.write_fmt(args),
                En::D(writee) => writee.write_fmt(args),
                En::E(writee) => writee.write_fmt(args),
                En::F(writee) => writee.write_fmt(args),
                En::G(writee) => writee.write_fmt(args),
                En::H(writee) => writee.write_fmt(args),
                En::I(writee) => writee.write_fmt(args),
                En::J(writee) => writee.write_fmt(args),
                En::K(writee) => writee.write_fmt(args),
                En::L(writee) => writee.write_fmt(args),
            }
        }
    }
}

mod iter {
    use crate::*;

    use core::iter::{
        FusedIterator,
        DoubleEndedIterator,
        ExactSizeIterator,

        Extend,
    };

    impl<A: Iterator> Iterator for En<A> {
        type Item = A::Item;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                En::A(iter) => iter.next(),
                _ => unreachable!(),
            }    
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                En::A(iter) => iter.size_hint(),
                _ => unreachable!(),
            } 
        }

        fn count(self) -> usize {
            match self {
                En::A(iter) => iter.count(),
                _ => unreachable!()
            }
        }

        fn last(self) -> Option<Self::Item> {
            match self {
                En::A(iter) => iter.last(),
                _ => unreachable!()
            }
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                En::A(iter) => iter.nth(n),
                _ => unreachable!()
            }
        }
    }

    impl<A: DoubleEndedIterator> DoubleEndedIterator for En<A> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                En::A(iter) => iter.next_back(),
                _ => unreachable!()
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            match self {
                En::A(iter) => iter.nth_back(n),
                _ => unreachable!()
            }
        }
    }

    impl<A: FusedIterator> FusedIterator for En<A> { }

    impl<A: ExactSizeIterator> ExactSizeIterator for En<A> {
        fn len(&self) -> usize {
            match self {
                En::A(iter) => iter.len(),
                _ => unreachable!()
            }
        }
    }

    impl<Item,
        A: Extend<Item>,
        B: Extend<Item>,
        C: Extend<Item>,
        D: Extend<Item>,
        E: Extend<Item>,
        F: Extend<Item>,
        G: Extend<Item>,
        H: Extend<Item>,
        I: Extend<Item>,
        J: Extend<Item>,
        K: Extend<Item>,
        L: Extend<Item>,
    > Extend<Item> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
        fn extend<Iter: IntoIterator<Item = Item>>(&mut self, iter: Iter) {
            match self {
                En::A(extendee) => extendee.extend(iter),
                _ => unreachable!(),
            }
        }
    }
    
    macro_rules! impl_iter_for_en {
        ( $($ty:ident)* ) => {
            impl<
                A: Iterator,
                $(
                    $ty: Iterator,
                )*
            > Iterator for En<A, $($ty, )*> {
                type Item = En<A::Item, $($ty::Item, )*>;
    
                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        En::A(iter) => iter.next().map(En::A),
                        $(
                            En::$ty(iter) => iter.next().map(En::$ty),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
    
                fn size_hint(&self) -> (usize, Option<usize>) {
                    match self {
                        En::A(iter) => iter.size_hint(),
                        $(
                            En::$ty(iter) => iter.size_hint(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
    
                fn count(self) -> usize {
                    match self {
                        En::A(iter) => iter.count(),
                        $(
                            En::$ty(iter) => iter.count(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
    
                fn last(self) -> Option<Self::Item> {
                    match self {
                        En::A(iter) => iter.last().map(En::A),
                        $(
                            En::$ty(iter) => iter.last().map(En::$ty),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
    
                fn nth(&mut self, n: usize) -> Option<Self::Item> {
                    match self {
                        En::A(iter) => iter.nth(n).map(En::A),
                        $(
                            En::$ty(iter) => iter.nth(n).map(En::$ty),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }
    
            impl<
                A: DoubleEndedIterator,
                $(
                    $ty: DoubleEndedIterator,
                )*
            > DoubleEndedIterator for En<A, $($ty,)*> {
                fn next_back(&mut self) -> Option<Self::Item> {
                    match self {
                        En::A(iter) => iter.next_back().map(En::A),
                        $(
                            En::$ty(iter) => iter.next_back().map(En::$ty),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
    
                fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                    match self {
                        En::A(iter) => iter.nth_back(n).map(En::A),
                        $(
                            En::$ty(iter) => iter.nth_back(n).map(En::$ty),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }
    
            impl<
                A: FusedIterator,
                $(
                    $ty: FusedIterator,
                )*
            > FusedIterator for En<A, $($ty,)*> { }

            impl<
                A: ExactSizeIterator,
                $(
                    $ty: ExactSizeIterator,
                )*
            > ExactSizeIterator for En<A, $($ty,)*> {
                fn len(&self) -> usize {
                    match self {
                        En::A(iter) => iter.len(),
                        $(
                            En::$ty(iter) => iter.len(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }
        };
    }
    
    impl_iter_for_en!{B }
    impl_iter_for_en!{B C}
    impl_iter_for_en!{B C D}
    impl_iter_for_en!{B C D E}
    impl_iter_for_en!{B C D E F}
    impl_iter_for_en!{B C D E F G}
    impl_iter_for_en!{B C D E F G H}
    impl_iter_for_en!{B C D E F G H I}
    impl_iter_for_en!{B C D E F G H I J}
    impl_iter_for_en!{B C D E F G H I J K}
    impl_iter_for_en!{B C D E F G H I J K L}
}

#[cfg(feature = "async")]
mod future {
    use crate::*;

    use core::{
        future::Future,
        task::{Context, Poll},
        pin::{Pin, pin},
    };

    macro_rules! impl_future_for_en {
        ( $($ty:ident)+ ) => {
            impl<
                $(
                    $ty: Future + Unpin,
                )*
            > Future for En<$($ty,)*>
            {
                type Output = En<$(<$ty as Future>::Output, )*>;

                fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
                    match self.get_mut() {
                        $(
                            En::$ty(val) => pin!(val).poll(ctx).map(En::$ty),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
            }
        };
    }
    
    impl<A: Future + Unpin> Future for En<A> {
        type Output = A::Output;

        fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
            let En::A(a) = self.get_mut() else {
                unreachable!()
            };

            pin!(a).poll(ctx)
        }
    }

    impl_future_for_en!{A B }
    impl_future_for_en!{A B C}
    impl_future_for_en!{A B C D}
    impl_future_for_en!{A B C D E}
    impl_future_for_en!{A B C D E F}
    impl_future_for_en!{A B C D E F G}
    impl_future_for_en!{A B C D E F G H}
    impl_future_for_en!{A B C D E F G H I}
    impl_future_for_en!{A B C D E F G H I J}
    impl_future_for_en!{A B C D E F G H I J K}
    impl_future_for_en!{A B C D E F G H I J K L}
}

#[cfg(feature = "std")]
mod io {
    use crate::*;

    use std::io::{
        Read,
        Seek,
        Write,
        BufRead,

        SeekFrom,

        Result,
    };

    macro_rules! impl_io_for_en {
        ( $($ty:ident)+ ) => {
            impl<$($ty: Read, )+> Read for En<$($ty, )+> {
                fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read_vectored(bufs),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                #[cfg(false)]
                fn is_read_vectored(&self) -> bool {
                    match self {
                        $(
                            En::$ty(reader) => reader.is_read_vectored(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn read_to_end(&mut self, buf: &mut alloc::vec::Vec<u8>) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn read_to_string(&mut self, buf: &mut alloc::string::String) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read_to_string(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read_exact(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                #[cfg(false)]
                fn read_buf(&mut self, buf: std::io::BorrowedCursor<'_>) -> Result<()> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read_buf(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                #[cfg(false)]
                fn read_buf_exact(&mut self, cursor: std::io::BorrowedCursor<'_>) -> Result<()> {
                    match self {
                        $(
                            En::$ty(reader) => reader.read_buf_exact(cursor),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                // fn by_ref(&mut self) -> &mut Self { ... }
                // fn bytes(self) -> Bytes<Self> { ... }
                // fn chain<R: Read>(self, next: R) -> Chain<Self, R> { ... }
                // fn take(self, limit: u64) -> Take<Self> { ... }
            }

            impl<$($ty: Seek, )+> Seek for En<$($ty, )+> {
                fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.seek(pos),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                fn rewind(&mut self) -> Result<()> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.rewind(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                #[cfg(false)]
                fn stream_len(&mut self) -> Result<u64> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.stream_len(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn stream_position(&mut self) -> Result<u64> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.stream_position(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn seek_relative(&mut self, offset: i64) -> Result<()> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.seek_relative(offset),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
            }

            impl<$($ty: Write, )+> Write for En<$($ty, )+> {
                fn write(&mut self, buf: &[u8]) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(writer) => writer.write(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                fn flush(&mut self) -> Result<()> {
                    match self {
                        $(
                            En::$ty(writer) => writer.flush(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(writer) => writer.write_vectored(bufs),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                #[cfg(false)]
                fn is_write_vectored(&self) -> bool {
                    match self {
                        $(
                            En::$ty(writer) => writer.is_write_vectored(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                fn write_all(&mut self, buf: &[u8]) -> Result<()> {
                    match self {
                        $(
                            En::$ty(writer) => writer.write_all(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                #[cfg(false)]
                fn write_all_vectored(&mut self, bufs: &mut [std::io::IoSlice<'_>]) -> Result<()> {
                    match self {
                        $(
                            En::$ty(writer) => writer.write_all_vectored(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<()> {
                    match self {
                        $(
                            En::$ty(writer) => writer.write_fmt(args),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
            }

            impl<$($ty: BufRead, )+> BufRead for En<$($ty, )+> {
                fn fill_buf(&mut self) -> Result<&[u8]> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.fill_buf(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }

                fn consume(&mut self, amount: usize) {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.consume(amount),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                #[cfg(false)]
                fn has_data_left(&mut self) -> Result<bool> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.has_data_left(),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn read_until(&mut self, byte: u8, buf: &mut alloc::vec::Vec<u8>) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.read_until(byte, buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn skip_until(&mut self, byte: u8) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.skip_until(byte),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                fn read_line(&mut self, buf: &mut alloc::string::String) -> Result<usize> {
                    match self {
                        $(
                            En::$ty(seeker) => seeker.read_line(buf),
                        )*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
                
                // fn split(self, byte: u8) -> std::io::Split<Self> { ... }
                // fn lines(self) -> std::io::Lines<Self> { ... }
                
            }
        };
    }
    
    impl_io_for_en!{A }
    impl_io_for_en!{A B }
    impl_io_for_en!{A B C}
    impl_io_for_en!{A B C D}
    impl_io_for_en!{A B C D E}
    impl_io_for_en!{A B C D E F}
    impl_io_for_en!{A B C D E F G}
    impl_io_for_en!{A B C D E F G H}
    impl_io_for_en!{A B C D E F G H I}
    impl_io_for_en!{A B C D E F G H I J}
    impl_io_for_en!{A B C D E F G H I J K}
    impl_io_for_en!{A B C D E F G H I J K L}
}

mod ops {
    use core::ops::RangeBounds;
    use crate::*;

    impl<
        T: ?Sized,
        A: RangeBounds<T>,
        B: RangeBounds<T>,
        C: RangeBounds<T>,
        D: RangeBounds<T>,
        E: RangeBounds<T>,
        F: RangeBounds<T>,
        G: RangeBounds<T>,
        H: RangeBounds<T>,
        I: RangeBounds<T>,
        J: RangeBounds<T>,
        K: RangeBounds<T>,
        L: RangeBounds<T>,
    > RangeBounds<T> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
        fn start_bound(&self) -> core::ops::Bound<&T> {
            match self {
                En::A(range) => range.start_bound(),
                En::B(range) => range.start_bound(),
                En::C(range) => range.start_bound(),
                En::D(range) => range.start_bound(),
                En::E(range) => range.start_bound(),
                En::F(range) => range.start_bound(),
                En::G(range) => range.start_bound(),
                En::H(range) => range.start_bound(),
                En::I(range) => range.start_bound(),
                En::J(range) => range.start_bound(),
                En::K(range) => range.start_bound(),
                En::L(range) => range.start_bound(),
            }
        }

        fn end_bound(&self) -> core::ops::Bound<&T> {
            match self {
                En::A(range) => range.end_bound(),
                En::B(range) => range.end_bound(),
                En::C(range) => range.end_bound(),
                En::D(range) => range.end_bound(),
                En::E(range) => range.end_bound(),
                En::F(range) => range.end_bound(),
                En::G(range) => range.end_bound(),
                En::H(range) => range.end_bound(),
                En::I(range) => range.end_bound(),
                En::J(range) => range.end_bound(),
                En::K(range) => range.end_bound(),
                En::L(range) => range.end_bound(),
            }
        }

        fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            match self {
                En::A(range) => range.contains(item),
                En::B(range) => range.contains(item),
                En::C(range) => range.contains(item),
                En::D(range) => range.contains(item),
                En::E(range) => range.contains(item),
                En::F(range) => range.contains(item),
                En::G(range) => range.contains(item),
                En::H(range) => range.contains(item),
                En::I(range) => range.contains(item),
                En::J(range) => range.contains(item),
                En::K(range) => range.contains(item),
                En::L(range) => range.contains(item),
            }
        }

        #[cfg(false)]
        fn is_empty(&self) -> bool
        where T: PartialOrd
        {
            match self {
                En::A(range) => range.is_empty(),
                En::B(range) => range.is_empty(),
                En::C(range) => range.is_empty(),
                En::D(range) => range.is_empty(),
                En::E(range) => range.is_empty(),
                En::F(range) => range.is_empty(),
                En::G(range) => range.is_empty(),
                En::H(range) => range.is_empty(),
                En::I(range) => range.is_empty(),
                En::J(range) => range.is_empty(),
                En::K(range) => range.is_empty(),
                En::L(range) => range.is_empty(),
            }
        }
    }

    #[cfg(false)]
    mod operator_impls {}

    mod taking_impls {
        use crate::*;

        use core::{
            // ops::{
            //     Deref,
            //     DerefMut,
            // },
            convert::{
                AsRef,
                AsMut,
            },
            // borrow::{
            //     Borrow,
            //     BorrowMut,
            // },
        };

        impl<
            Item: ?Sized,
            A: AsRef<Item>,
            B: AsRef<Item>,
            C: AsRef<Item>,
            D: AsRef<Item>,
            E: AsRef<Item>,
            F: AsRef<Item>,
            G: AsRef<Item>,
            H: AsRef<Item>,
            I: AsRef<Item>,
            J: AsRef<Item>,
            K: AsRef<Item>,
            L: AsRef<Item>,
        > AsRef<Item> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
            fn as_ref(&self) -> &Item {
                match self {
                    En::A(rf) => rf.as_ref(),
                    En::B(rf) => rf.as_ref(),
                    En::C(rf) => rf.as_ref(),
                    En::D(rf) => rf.as_ref(),
                    En::E(rf) => rf.as_ref(),
                    En::F(rf) => rf.as_ref(),
                    En::G(rf) => rf.as_ref(),
                    En::H(rf) => rf.as_ref(),
                    En::I(rf) => rf.as_ref(),
                    En::J(rf) => rf.as_ref(),
                    En::K(rf) => rf.as_ref(),
                    En::L(rf) => rf.as_ref(),
                }
            }
        }

        impl<
            Item: ?Sized,
            A: AsMut<Item>,
            B: AsMut<Item>,
            C: AsMut<Item>,
            D: AsMut<Item>,
            E: AsMut<Item>,
            F: AsMut<Item>,
            G: AsMut<Item>,
            H: AsMut<Item>,
            I: AsMut<Item>,
            J: AsMut<Item>,
            K: AsMut<Item>,
            L: AsMut<Item>,
        > AsMut<Item> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
            fn as_mut(&mut self) -> &mut Item {
                match self {
                    En::A(rf) => rf.as_mut(),
                    En::B(rf) => rf.as_mut(),
                    En::C(rf) => rf.as_mut(),
                    En::D(rf) => rf.as_mut(),
                    En::E(rf) => rf.as_mut(),
                    En::F(rf) => rf.as_mut(),
                    En::G(rf) => rf.as_mut(),
                    En::H(rf) => rf.as_mut(),
                    En::I(rf) => rf.as_mut(),
                    En::J(rf) => rf.as_mut(),
                    En::K(rf) => rf.as_mut(),
                    En::L(rf) => rf.as_mut(),
                }
            }
        }

        #[cfg(false)]
        #[cfg(feature = "nightly_internals")]
        impl<
            Item: ?Sized,
            A: Borrow<Item>,
            B: Borrow<Item>,
            C: Borrow<Item>,
            D: Borrow<Item>,
            E: Borrow<Item>,
            F: Borrow<Item>,
            G: Borrow<Item>,
            H: Borrow<Item>,
            I: Borrow<Item>,
            J: Borrow<Item>,
            K: Borrow<Item>,
            L: Borrow<Item>,
        > Borrow<Item> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
            fn borrow(&self) -> &Item {
                match self {
                    En::A(rf) => rf.borrow(),
                    En::B(rf) => rf.borrow(),
                    En::C(rf) => rf.borrow(),
                    En::D(rf) => rf.borrow(),
                    En::E(rf) => rf.borrow(),
                    En::F(rf) => rf.borrow(),
                    En::G(rf) => rf.borrow(),
                    En::H(rf) => rf.borrow(),
                    En::I(rf) => rf.borrow(),
                    En::J(rf) => rf.borrow(),
                    En::K(rf) => rf.borrow(),
                    En::L(rf) => rf.borrow(),
                }
            }
        }

        #[cfg(false)]
        #[cfg(feature = "nightly_internals")]
        impl<
            Item: ?Sized,
            A: BorrowMut<Item>,
            B: BorrowMut<Item>,
            C: BorrowMut<Item>,
            D: BorrowMut<Item>,
            E: BorrowMut<Item>,
            F: BorrowMut<Item>,
            G: BorrowMut<Item>,
            H: BorrowMut<Item>,
            I: BorrowMut<Item>,
            J: BorrowMut<Item>,
            K: BorrowMut<Item>,
            L: BorrowMut<Item>,
        > BorrowMut<Item> for En<A, B, C, D, E, F, G, H, I, J, K, L> {
            fn borrow_mut(&mut self) -> &mut Item {
                match self {
                    En::A(rf) => rf.borrow_mut(),
                    En::B(rf) => rf.borrow_mut(),
                    En::C(rf) => rf.borrow_mut(),
                    En::D(rf) => rf.borrow_mut(),
                    En::E(rf) => rf.borrow_mut(),
                    En::F(rf) => rf.borrow_mut(),
                    En::G(rf) => rf.borrow_mut(),
                    En::H(rf) => rf.borrow_mut(),
                    En::I(rf) => rf.borrow_mut(),
                    En::J(rf) => rf.borrow_mut(),
                    En::K(rf) => rf.borrow_mut(),
                    En::L(rf) => rf.borrow_mut(),
                }
            }
        }

        // macro_rules! impl_take_for_en {
        //     ( $($ty:ident)+ ) => {
        //         // impl<
        //         //     Item: ?Sized,
        //         //     $(
        //         //         $ty: AsRef<Item>,
        //         //     )+
        //         // > AsRef<Item> for En<$($ty, )+> {
        //         //     fn as_ref(&self) -> &Item {
        //         //         match self {
        //         //             $(
        //         //                 En::$ty(rf) => rf.as_ref(),
        //         //             )+
        //         //             #[allow(unreachable_patterns)]
        //         //             _ => unreachable!(),
        //         //         }
        //         //     }
        //         // }
        //     };
        // }

        // impl_take_for_en!{A }
        // impl_take_for_en!{A B }
        // impl_take_for_en!{A B C}
        // impl_take_for_en!{A B C D}
        // impl_take_for_en!{A B C D E}
        // impl_take_for_en!{A B C D E F}
        // impl_take_for_en!{A B C D E F G}
        // impl_take_for_en!{A B C D E F G H}
        // impl_take_for_en!{A B C D E F G H I}
        // impl_take_for_en!{A B C D E F G H I J}
        // impl_take_for_en!{A B C D E F G H I J K}
        // impl_take_for_en!{A B C D E F G H I J K L}
    }

    #[cfg(feature = "fn_impls")]
    mod fn_impls {
        use crate::*;
        
        use core::marker::Tuple;

        impl<Args: Tuple, A: FnOnce<Args>> FnOnce<Args> for En<A> {
            type Output = A::Output;

            extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
                match self {
                    En::A(func) => func.call_once(args)
                }
            }
        }

        impl<Args: Tuple, A: FnMut<Args>> FnMut<Args> for En<A> {
            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
                match self {
                    En::A(func) => func.call_mut(args),
                    _ => unreachable!(),
                }
            }
        }

        impl<Args: Tuple, A: Fn<Args>> Fn<Args> for En<A> {
            extern "rust-call" fn call(&self, args: Args) -> Self::Output {
                match self {
                    En::A(func) => func.call(args),
                    _ => unreachable!(),
                }
            }
        }

        #[cfg(feature = "async")]
        impl<
            Args: Tuple,
            A: AsyncFnOnce<Args>
        > AsyncFnOnce<Args> for En<A> {
            type Output = A::Output;
            type CallOnceFuture = A::CallOnceFuture;

            extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
                match self {
                    En::A(func) => func.async_call_once(args)
                }
            }
        }

        #[cfg(feature = "async")]
        impl<
            Args: Tuple,
            A: AsyncFnMut<Args>,
        > AsyncFnMut<Args> for En<A> {
            type CallRefFuture<'a> = A::CallRefFuture<'a> where A: 'a;

            extern "rust-call" fn async_call_mut<'a>(&'a mut self, args: Args) -> Self::CallRefFuture<'a> {
                match self {
                    En::A(func) => func.async_call_mut(args),
                    _ => unreachable!(),
                }
            }
        }

        #[cfg(feature = "async")]
        impl<Args: Tuple, A: AsyncFn<Args>> AsyncFn<Args> for En<A> {
            extern "rust-call" fn async_call<'a>(&'a self, args: Args) -> Self::CallRefFuture<'a> {
                match self {
                    En::A(func) => func.async_call(args),
                    _ => unreachable!(),
                }
            }
        }

        macro_rules! impl_fn_for_en {
            ( $($ty:ident)+ ) => {
                impl<
                    Args: Tuple,
                    A: FnOnce<Args>,
                    $(
                        $ty: FnOnce<Args> + NotNever,
                    )+
                > FnOnce<Args> for En<A, $($ty, )+> {
                    type Output = En<A::Output, $($ty::Output, )+>;

                    extern "rust-call" fn call_once(self, args: Args) -> En<A::Output, $($ty::Output, )+> {
                        match self {
                            En::A(func) => En::A(func.call_once(args)),
                            $(
                                En::$ty(func) => En::$ty(func.call_once(args)),
                            )+
                            #[allow(unreachable_patterns)]
                            _ => unreachable!(),
                        }
                    }
                }
                
                impl<
                    Args: Tuple,
                    A: FnMut<Args>,
                    $(
                        $ty: FnMut<Args> + NotNever,
                    )+
                > FnMut<Args> for En<A, $($ty, )+> {
                    extern "rust-call" fn call_mut(&mut self, args: Args) -> En<A::Output, $($ty::Output, )+> {
                        match self {
                            En::A(func) => En::A(func.call_mut(args)),
                            $(
                                En::$ty(func) => En::$ty(func.call_mut(args)),
                            )+
                            #[allow(unreachable_patterns)]
                            _ => unreachable!(),
                        }
                    }
                }
                
                impl<
                    Args: Tuple,
                    A: Fn<Args>,
                    $(
                        $ty: Fn<Args> + NotNever,
                    )+
                > Fn<Args> for En<A, $($ty, )+> {
                    extern "rust-call" fn call(&self, args: Args) -> En<A::Output, $($ty::Output, )+> {
                        match self {
                            En::A(func) => En::A(func.call(args)),
                            $(
                                En::$ty(func) => En::$ty(func.call(args)),
                            )+
                            #[allow(unreachable_patterns)]
                            _ => unreachable!(),
                        }
                    }
                }
                
                #[cfg(feature = "async")]
                impl<
                    Args: Tuple,
                    A: AsyncFnOnce<Args>,
                    $(
                        $ty: AsyncFnOnce<Args> + NotNever,
                    )+
                > AsyncFnOnce<Args> for En<A, $($ty, )+>
                where En<A::CallOnceFuture, $($ty::CallOnceFuture, )+>: core::future::Future<Output = En<A::Output, $($ty::Output, )+>>,
                {
                    type Output = En<A::Output, $($ty::Output, )+>;
                    type CallOnceFuture = En<A::CallOnceFuture, $($ty::CallOnceFuture, )+>;

                    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
                        match self {
                            En::A(func) => En::A(func.async_call_once(args)),
                            $(
                                En::$ty(func) => En::$ty(func.async_call_once(args)),
                            )+
                            #[allow(unreachable_patterns)]
                            _ => unreachable!(),
                        }
                    }
                }
                
                #[cfg(feature = "async")]
                impl<
                    Args: Tuple,
                    A: AsyncFnMut<Args>,
                    $(
                        $ty: AsyncFnMut<Args> + NotNever,
                    )+
                > AsyncFnMut<Args> for En<A, $($ty, )+>
                where
                    En<A::CallOnceFuture, $($ty::CallOnceFuture, )+>: core::future::Future<Output = En<A::Output, $($ty::Output, )+>>,
                    for<'en> En<A::CallRefFuture<'en>, $($ty::CallRefFuture<'en>, )+>: core::future::Future<Output = En<A::Output, $($ty::Output,)+>>,
                {
                    type CallRefFuture<'a> = En<A::CallRefFuture<'a>, $($ty::CallRefFuture<'a>, )+>
                    where
                        A: 'a, 
                        $(
                            $ty: 'a,
                        )+
                    ;

                    extern "rust-call" fn async_call_mut<'a>(&'a mut self, args: Args) -> Self::CallRefFuture<'a> {
                        match self {
                            En::A(func) => En::A(func.async_call_mut(args)),
                            $(
                                En::$ty(func) => En::$ty(func.async_call_mut(args)),
                            )+
                            #[allow(unreachable_patterns)]
                            _ => unreachable!(),
                        }
                    }
                }
                
                #[cfg(feature = "async")]
                impl<
                    Args: Tuple,
                    A: AsyncFn<Args>,
                    $(
                        $ty: AsyncFn<Args> + NotNever,
                    )+
                > AsyncFn<Args> for En<A, $($ty, )+>
                where
                    En<A::CallOnceFuture, $($ty::CallOnceFuture, )+>: core::future::Future<Output = En<A::Output, $($ty::Output, )+>>,
                    for<'en> En<A::CallRefFuture<'en>, $($ty::CallRefFuture<'en>, )+>: core::future::Future<Output = En<A::Output, $($ty::Output,)+>>,
                {
                    extern "rust-call" fn async_call<'a>(&'a self, args: Args) -> Self::CallRefFuture<'a> {
                        match self {
                            En::A(func) => En::A(func.async_call(args)),
                            $(
                                En::$ty(func) => En::$ty(func.async_call(args)),
                            )+
                            #[allow(unreachable_patterns)]
                            _ => unreachable!(),
                        }
                    }
                }
            };
        }

        impl_fn_for_en!{B }
        impl_fn_for_en!{B C}
        impl_fn_for_en!{B C D}
        impl_fn_for_en!{B C D E}
        impl_fn_for_en!{B C D E F}
        impl_fn_for_en!{B C D E F G}
        impl_fn_for_en!{B C D E F G H}
        impl_fn_for_en!{B C D E F G H I}
        impl_fn_for_en!{B C D E F G H I J}
        impl_fn_for_en!{B C D E F G H I J K}
        impl_fn_for_en!{B C D E F G H I J K L}
    }
}

#[cfg(feature = "serde")]
mod serde {
    use crate::*;

    use serde::{
        Deserialize, Deserializer, de
    };

    impl<
        A: serde::Serialize,
        B: serde::Serialize,
        C: serde::Serialize,
        D: serde::Serialize,
        E: serde::Serialize,
        F: serde::Serialize,
        G: serde::Serialize,
        H: serde::Serialize,
        I: serde::Serialize,
        J: serde::Serialize,
        K: serde::Serialize,
        L: serde::Serialize,
    > serde::Serialize for En<A, B, C, D, E, F, G, H, I, J, K, L> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
        {
            match self {
                En::A(value) => value.serialize(serializer),
                En::B(value) => value.serialize(serializer),
                En::C(value) => value.serialize(serializer),
                En::D(value) => value.serialize(serializer),
                En::E(value) => value.serialize(serializer),
                En::F(value) => value.serialize(serializer),
                En::G(value) => value.serialize(serializer),
                En::H(value) => value.serialize(serializer),
                En::I(value) => value.serialize(serializer),
                En::J(value) => value.serialize(serializer),
                En::K(value) => value.serialize(serializer),
                En::L(value) => value.serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for En {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(de::Error::custom("cannot deserialize `!`"))
        }
    }

    impl<'de, A: Deserialize<'de>> Deserialize<'de> for En<A> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            A::deserialize(deserializer).map(En::A)
        }
    }

    #[cfg(false)]
    impl<'de, A: Deserialize<'de>, B: Deserialize<'de>> Deserialize<'de> for En<A, B> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>,
        {
            if core::any::TypeId::of::<A>() == core::any::TypeId::of::<B>() {
                return Err(de::Error::invalid_type(de::Unexpected::Other("Same type as `A`"), &"Can't parse a choice of 2 of the same tyoe"))
            }

            todo!()
        }
    }
}
