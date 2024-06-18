//! Implementation crate for the frozen collections.
//!
//! Application code should generally not interact with
//! this crate. Please use
//! the `frozen-collections` crate instead.

use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;

use frozen_collections_core::macros::{frozen_map_macro, frozen_set_macro};

#[proc_macro]
#[proc_macro_error]
pub fn frozen_map(item: TokenStream) -> TokenStream {
    frozen_map_macro(item.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn frozen_set(item: TokenStream) -> TokenStream {
    frozen_set_macro(item.into()).into()
}
