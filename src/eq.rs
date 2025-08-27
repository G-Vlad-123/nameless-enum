
use core::marker::PhantomData;

struct ConstraintInner<A: ?Sized, B: ?Sized> {
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

pub struct Constraint<A: ?Sized, B: ?Sized> {
    _inner: ConstraintInner<A, B>
}

pub auto trait SameTy {}

impl<A: ?Sized, B: ?Sized> !SameTy for ConstraintInner<A, B> {}

impl<T: ?Sized> SameTy for Constraint<T, T> {}

pub trait Unique {}

// impl<A: ?Sized, B: ?Sized> !Unique for ConstraintInner<A, B> {}

default impl<A: ?Sized, B: ?Sized> Unique for Constraint<A, B> {}
impl<A: ?Sized> !Unique for Constraint<A, A> {}

// pub enum SameTy {}
// pub enum Unique {}

// trait EqTy {}

// impl EqTy for SameTy {}
// impl EqTy for Unique {}

// pub trait Check<Other: ?Sized> {
//     type EqTy: EqTy;
// }

// impl<A: ?Sized, B: ?Sized> Check<B> for A {
//     default type EqTy = Unique;
// }

// impl<T: ?Sized> Check<T> for T {
//     type EqTy = SameTy;
// }

// pub trait Is<Other: ?Sized>
// where
//     Self: Check<Other, EqTy = SameTy>,
// {}

// pub trait IsNot<Other: ?Sized>
// where
//     Self: Check<Other, EqTy = Unique>,
// {}

// fn test<A, B>(a: A, b: B) where Constraint<A, B>: !SameTy {}

// fn test_2() {
//     test(1, "hi")
// }
