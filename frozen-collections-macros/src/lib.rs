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
pub fn fz_scalar_map(item: TokenStream) -> TokenStream {
    fz_scalar_map_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn fz_scalar_set(item: TokenStream) -> TokenStream {
    fz_scalar_set_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[proc_macro_derive(Scalar)]
#[proc_macro_error]
pub fn derive_scalar(item: TokenStream) -> TokenStream {
    derive_scalar_macro(item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
