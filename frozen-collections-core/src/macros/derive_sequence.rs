use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Variant};

/// Implementation logic for the `Sequence` derive macro.
///
/// # Errors
///
/// Bad things happen to bad input
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::missing_panics_doc)]
pub fn derive_sequence_macro(args: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(args)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Data::Enum(variants) = &input.data else {
        return Err(Error::new_spanned(
            name,
            "Sequence can only be used with enums",
        ));
    };

    if variants.variants.is_empty() {
        return Err(Error::new_spanned(
            name,
            "Sequence can only be used with non-empty enums",
        ));
    }

    // we make sure there is a unit variant at the start and end of the
    // sequence, such that we can express the Sequence::MIN/MAX constants.
    let mut variants: Vec<&Variant> = variants.variants.iter().collect();
    variants.sort_by_key(|v| match v.fields {
        Fields::Unit => 0,
        Fields::Unnamed(..) => 1,
        Fields::Named(..) => 2,
    });

    let mut num_unit_variants = 0;
    for variant in &variants {
        if variant.fields != Fields::Unit {
            break;
        }

        num_unit_variants += 1;
    }

    if variants.len() == 1 {
        if num_unit_variants != 1 {
            return Err(Error::new_spanned(
                input,
                "Sequence can only be used with singleton enums having a unit enum variant",
            ));
        }
    } else {
        if num_unit_variants < 2 {
            return Err(Error::new_spanned(
                input,
                "Sequence can only be used with enums having a minimum of 2 unit variants",
            ));
        }

        // put a unit variant at the end of the sequence
        let len = variants.len();
        variants.swap(1, len - 1);
    }

    let mut matches = Vec::new();
    for variant in &variants {
        let ident = &variant.ident;

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        let index = matches.len();
        matches.push(quote! { #name::#ident #params => #index});
    }

    let first_variant = &variants
        .first()
        .expect("variant array should not be empty")
        .ident;
    let last_variant = &variants
        .last()
        .expect("variant array should not be empty")
        .ident;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline]
            const fn to_usize(&self) -> usize {
                match self {
                    #(#matches),*
                }
            }
        }

        #[automatically_derived]
        impl #impl_generics ::frozen_collections::Sequence for #name #ty_generics #where_clause {
            const MIN: Self = Self::#first_variant;
            const MAX: Self = Self::#last_variant;

            fn as_u64(&self) -> u64 {
                self.to_usize() as u64
            }

            fn offset(min: &Self, max:&Self, value: &Self) -> Option<usize> {
                let max_index = max.to_usize().wrapping_sub(min.to_usize());
                let value_index = value.to_usize().wrapping_sub(min.to_usize());

                if value_index > max_index {
                    None
                } else {
                    Some(value_index)
                }
            }

            fn count(min: &Self, max: &Self) -> Option<usize> {
                let min = min.to_usize();
                let max = max.to_usize();

                if max < min {
                    None
                } else if max.abs_diff(min) as u128 > usize::MAX as u128 {
                    None
                } else {
                    Some(max.abs_diff(min) as usize + 1)
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
#[derive(Sequence, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Color {
    Red(bool),
}
",
        )
        .unwrap();

        let ts2 = derive_sequence_macro(ts);

        println!("{ts2:?}");
    }
}
