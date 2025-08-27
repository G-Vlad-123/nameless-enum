
extern crate syn;
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;

#[proc_macro_attribute]
#[doc = include_str!("../../docs/feature_choice.md")]
pub fn feature_choice(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
