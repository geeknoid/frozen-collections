use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields};

/// Implementation logic for the `Scalar` derive macro.
///
/// # Errors
///
/// Bad things happen to bad input
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::missing_panics_doc)]
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
                "Scalar can only be used with enums that only contains unit variants",
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

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        let index = matches.len();
        matches.push(quote! { #name::#ident #params => #index});
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
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    use super::*;

    #[test]
    fn basic() {
        let ts = TokenStream::from_str(
            "
#[derive(Scalar, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Color {
    Red(bool),
}
",
        )
        .unwrap();

        let ts2 = derive_scalar_macro(ts);

        println!("{ts2:?}");
    }
}
