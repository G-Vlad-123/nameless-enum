A library for the anonymous enum (maybe future rfc 🤞) proposal.

# A Quick Overview
This crate provides an anonymous enum
as a proof of concept to (maybe) be added
to rustc in the future.

These anonymous enums are are named choices as they
provide a choice between multiple possible types
(The name is not set in stone), whenever refering to the possible
version of an anonymous enum that might end up in rustc they will bre referd to
as "a choice" meanwhile the provided anonymous enum by teh crate will be refered
to as "a Choice enum", to distinguish between the two more easely.

Choices would be an in-build language construct that might use the following syntax:
`Type1 | Type2 | ... | TypeN` for up to `N` types.
Currently the provided `Choice` enum
enum can only support up to 12 types but this can be coutnered by nesting Choice enums them.

Choices can be also tought of as any of these:
- The enum equivalent to what [`tuple`s](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) are for [`struct`s](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html).
- The type version of [`dyn Trait`](https://quinedot.github.io/rust-learning/dyn-trait.html) without a v-table.
- The type version of [`enum-dispatch`](https://docs.rs/enum_dispatch/latest/enum_dispatch/index.html).

# How to use

## Enabeling

Currently the [`feature_choices`] macro does nothing but when finished this is how you would enable choices:

Add this to your `Cargo.toml` file:
```
[dependencies.nameless-enum]
version = "*"
features = [ "std" ]
```
By default this crate does not implement the `std` feature, this is because:
- It does nothing
- The only default is `macros` at the moment, which while can be disabled, is ment to be used 99% of the time,
and to not have someone forget to add it as a feature. (this mey change in the future).

Then add this line to the top of your file:
```rust
#![nameless_enum::feature_choices]
```

And done!

But for now you can just use the provided [`Choice`] enum.

# Final Notes

## Version `1.0.0`
Since this crate might be an rfc and is ment to be built as if it were a nightly
feature, (unless the RFC is made and denied with support from the comunity to keep
maintenence and a high enough level of quality and stability is reached)
this crate will never reach version `1.0.0`!

## RFC Creation
On a more personal note, I (the author of this crate) have not created an RFC before, and before I create
one I would like to:
- Learn more about the RFC creation process, how features are added, aproved, and what criteria is used
- See if this crate is usefull, what it brings and if it's worth continuing working on this project
- Finish the core build of this crate by adding most of what has to be added to the [`feature_choices`] macro.

If the RFC is ever created and aproved though this crate will have it's deprication planned, and it will be fully
depricated when the RFC is added to stable.

Apologies for any grammer mistakes, feal free to contribute to this crate by fixing them
(I would also include my discord userid to get in touch but I do not know as of writing
this if that is allowed) <3

(more docs will come in the future)
