use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields};

/// Implementation logic for the `Scalar` derive macro.
///
/// # Errors
///
/// Bad things happen to bad input
#[allow(clippy::module_name_repetitions)]
pub fn derive_scalar_macro(args: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(args)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Data::Enum(variants) = &input.data else {
        return Err(Error::new_spanned(
            name,
            "Scalar can only be used with enums",
        ));
    };

    if variants.variants.is_empty() {
        return Err(Error::new_spanned(
            name,
            "Scalar can only be used with non-empty enums",
        ));
    }

    for v in &variants.variants {
        if v.fields != Fields::Unit {
            return Err(Error::new_spanned(
                name,
                "Scalar can only be used with enums that only contain unit variants",
            ));
        }

        if v.discriminant.is_some() {
            return Err(Error::new_spanned(
                name,
                "Scalar can only be used with enums that do not have explicit discriminants",
            ));
        }
    }

    let mut matches = Vec::new();
    for variant in &variants.variants {
        let ident = &variant.ident;

        let index = matches.len();
        matches.push(quote! { #name::#ident => #index});
    }

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::frozen_collections::Scalar for #name #ty_generics #where_clause {
            fn index(&self) -> usize {
                match self {
                    #(#matches),*
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn basic() {
        assert!(
            derive_scalar_macro(quote!(
                enum Color {
                    Red,
                    Green,
                    Blue,
                }
            ))
            .is_ok()
        );
    }

    #[test]
    fn only_with_enums() {
        let r = derive_scalar_macro(quote!(
            struct Color {
                red: i32,
                green: i32,
                blue: i32,
            }
        ));

        assert_eq!(
            "Scalar can only be used with enums",
            r.unwrap_err().to_string()
        );
    }

    #[test]
    fn non_empty_enums() {
        let r = derive_scalar_macro(quote!(
            enum Color {}
        ));

        assert_eq!(
            "Scalar can only be used with non-empty enums",
            r.unwrap_err().to_string()
        );
    }

    #[test]
    fn only_unit_variants() {
        let r = derive_scalar_macro(quote!(
            enum Color {
                Red,
                Green(i32),
                Blue,
            }
        ));

        assert_eq!(
            "Scalar can only be used with enums that only contain unit variants",
            r.unwrap_err().to_string()
        );
    }

    #[test]
    fn no_explicit_discriminants() {
        let r = derive_scalar_macro(quote!(
            enum Color {
                Red,
                Green = 2,
                Blue,
            }
        ));

        assert_eq!(
            "Scalar can only be used with enums that do not have explicit discriminants",
            r.unwrap_err().to_string()
        );
    }
}
