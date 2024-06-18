//! Implementation crate for the frozen collections.
//!
//! # Compatibility Note
//!
//! This crate is not intended to be used directly. It is an implementation
//! detail of the `frozen-collections` crate. The API of this crate is therefore
//! not stable and may change at any time. If you need to use the functionality
//! of this crate, please use the `frozen-collections` crate instead which has
//! a stable API.

use frozen_collections_core::macros::*;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro]
#[proc_macro_error]
pub fn fz_hash_map(item: TokenStream) -> TokenStream {
    fz_hash_map_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_hash_set(item: TokenStream) -> TokenStream {
    fz_hash_set_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_ordered_map(item: TokenStream) -> TokenStream {
    fz_ordered_map_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_ordered_set(item: TokenStream) -> TokenStream {
    fz_ordered_set_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_string_map(item: TokenStream) -> TokenStream {
    fz_string_map_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_string_set(item: TokenStream) -> TokenStream {
    fz_string_set_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_sequence_map(item: TokenStream) -> TokenStream {
    fz_sequence_map_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_sequence_set(item: TokenStream) -> TokenStream {
    fz_sequence_set_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro_derive(Sequence)]
#[proc_macro_error]
pub fn derive_sequence(item: TokenStream) -> TokenStream {
    derive_sequence_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
