
#![doc = include_str!("../docs/lib.md")]

#![no_std]

// #![cfg_attr(feature = "specialization", feature(specialization))]

#![cfg_attr(feature = "nightly_internals", feature(negative_impls))]
#![cfg_attr(feature = "nightly_internals", feature(auto_traits))]

#![cfg_attr(feature = "fn_impls", feature(fn_traits))]
#![cfg_attr(feature = "fn_impls", feature(unboxed_closures))]
#![cfg_attr(feature = "fn_impls", feature(tuple_trait))]

#![cfg_attr(all(feature = "fn_impls", feature = "async"), feature(async_fn_traits))]

// #![feature(negative_impls)]
// #![feature(negative_bounds)]

// #![feature(specialization)]

// #![allow(internal_features)]

// pub mod eq;
// use eq::*;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "macros")]
extern crate nameless_enum_macros as macros;

#[doc(inline)]
#[cfg(feature = "macros")]
pub use macros::feature_choices;

mod never;
use never::Never;
#[cfg(feature = "nightly_internals")]
use never::NotNever;

#[non_exhaustive]
#[derive(Clone, Copy)]
#[allow(private_interfaces)]
#[doc = include_str!("../docs/Choice.md")]
pub enum Choice<
    A = Never,
    B = Never,
    C = Never,
    D = Never,
    E = Never,
    F = Never,
    G = Never,
    H = Never,
    I = Never,
    J = Never,
    K = Never,
    L = Never,
> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
    J(J),
    K(K),
    L(L),
}

/**
The exhsaustive version of [`Choice`]

This enum is not ment to be used directly,
and it's simpyl a helper for when using
the `#[exhsaustive]` attribute.

It's conversions from and into `Choice`
being automaticly added by the [`#[feature_choices]`](feature_choices) macro. 
*/
#[cfg_attr(false, derive(Clone, Copy))]
#[allow(private_interfaces)]
pub enum ExhaustiveChoice<
    A = Never,
    B = Never,
    C = Never,
    D = Never,
    E = Never,
    F = Never,
    G = Never,
    H = Never,
    I = Never,
    J = Never,
    K = Never,
    L = Never,
> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
    J(J),
    K(K),
    L(L),
}

use Choice as En;
use ExhaustiveChoice as ExEn;

mod std_impls;
