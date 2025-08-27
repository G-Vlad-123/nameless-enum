
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

For more detail check the documentation [`here`](target/doc/nameless_enum/index.html)!
