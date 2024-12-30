use crate::analyzers::{
    analyze_scalar_keys, analyze_slice_keys, ScalarKeyAnalysisResult, SliceKeyAnalysisResult,
};
use crate::emit::collection_entry::CollectionEntry;
use crate::emit::generator::{Generator, Output};
use crate::hashers::{LeftRangeHasher, PassthroughHasher, RightRangeHasher};
use crate::traits::Scalar;
use crate::utils::dedup_by_keep_last;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::hash::BuildHasher;
use foldhash::fast::RandomState;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Type, Visibility};

#[cfg(feature = "macros")]
use crate::emit::NonLiteralKey;

/// Emits frozen collection source code for use within a Rust build script.
///
/// This type makes it possible for a Rust build script to generate frozen collection
/// declarations. The build script can read data from arbitrary sources and then emit
/// it in the efficient frozen collection format. Analysis on the data is performed
/// automatically such that the generated frozen collection uses an optimized layout
/// and lookup algorithm.
///
/// # Example
///
/// ```no_run
/// # use frozen_collections_core::emit::*;
/// use std::env;
/// use std::fs::File;
/// use std::io::{BufWriter, Write};
/// use std::path::Path;
/// use syn::parse_quote;
///
/// fn main() {
///     let out_dir = env::var_os("OUT_DIR").unwrap();
///     let dest_path = Path::new(&out_dir).join("data.rs");
///     let mut file = BufWriter::new(File::create(dest_path).unwrap());
///
///     // Here's the set of entries that will be added to the generated
///     // collection. In this example, it's just a simple literal vector,
///     // but you can imagine getting this data from a JSON file or even
///     // from an online source.
///     let entries = vec![
///         CollectionEntry::map_entry("Hello", parse_quote! { "hello" }, parse_quote! { 42 }),
///         CollectionEntry::map_entry("World", parse_quote! { "world" }, parse_quote! { 24 }),
///     ];
///
///     // Create the emitter, configures it, and runs it. The result is a
///     // `TokenStream` which represents the code needed to create the
///     // frozen collection.
///     let map = CollectionEmitter::new(&parse_quote! { &'static str })
///         .value_type(&parse_quote! { i32 })
///         .symbol_name("MY_MAP")
///         .alias_name("MyMap")
///         .static_instance(true)
///         .emit_hash_collection(entries)
///         .unwrap();
///
///     // Write the generated frozen collection code to the file.
///     _ = writeln!(file, "{map}");
///
///     // given a directive to cargo to re-run the build script if it changes
///     println!("cargo::rerun-if-changed=build.rs");
/// }
/// ```
///
/// If you put the above in a `build.rs` file, Cargo will automatically execute it and
/// generate a file called `data.rs` in the `target` directory. You can then include
/// this file in your Rust code to access the generated frozen collection. For example:
///
/// ```ignore
/// include!(concat!(env!("OUT_DIR"), "/data.rs"));
///
/// fn main() {
///     println!("{MY_MAP:?}");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct CollectionEmitter {
    key_type: Type,
    pub(crate) value_type: Option<Type>,
    symbol_name: Option<String>,
    alias_name: Option<String>,
    const_keys: bool,
    const_values: bool,
    visibility: Visibility,
    is_mutable: bool,
    is_static: bool,

    #[allow(dead_code)]
    pub(crate) inferred_key_type: bool,

    #[allow(dead_code)]
    pub(crate) inferred_value_type: bool,
}

const SCAN_THRESHOLD: usize = 4;
const ORDERED_SCAN_THRESHOLD: usize = 7;
const BINARY_SEARCH_THRESHOLD: usize = 64;

impl CollectionEmitter {
    /// Creates a new `CollectionEmitter` instance.
    ///
    /// The `key_type` parameter specifies the type of the keys in the collection.
    /// For sets, this represents the type of value in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use frozen_collections_core::emit::CollectionEmitter;
    /// use syn::parse_quote;
    ///
    /// let emitter = CollectionEmitter::new(&parse_quote! { &'static str });
    /// ```
    #[must_use]
    pub fn new(key_type: &Type) -> Self {
        Self {
            key_type: key_type.clone(),
            value_type: None,
            symbol_name: None,
            alias_name: None,
            const_keys: false,
            const_values: false,
            visibility: Visibility::Inherited,
            is_static: false,
            is_mutable: false,
            inferred_key_type: false,
            inferred_value_type: false,
        }
    }

    #[cfg(feature = "macros")]
    #[must_use]
    pub(crate) fn new_with_inferred_key_type() -> Self {
        let mut result = Self::new(&syn::parse_quote! { _ });
        result.inferred_key_type = true;
        result
    }

    #[cfg(feature = "macros")]
    #[must_use]
    pub(crate) fn new_with_inferred_types() -> Self {
        let mut result = Self::new(&syn::parse_quote! { _ });
        result.value_type = Some(syn::parse_quote! { _ });
        result.inferred_key_type = true;
        result.inferred_value_type = true;
        result
    }

    /// When emitting a map, this specifies the types of the map's values.
    ///
    /// If the emitter's value type is not initialized, then the emitter will emit a set.
    #[must_use]
    pub fn value_type(mut self, value_type: &Type) -> Self {
        self.value_type = Some(value_type.clone());
        self
    }

    /// Specifies the name of the symbol to which the generated frozen collection will be assigned.
    ///
    /// If a symbol name isn't given, then the map will be emitted as an expression rather than
    /// a statement.
    #[must_use]
    pub fn symbol_name(mut self, symbol_name: &str) -> Self {
        self.symbol_name = Some(symbol_name.to_string());
        self
    }

    /// Specifies the name of the alias type that will be generated for the collection.
    ///
    /// This is the name that is used as an alias to the generated collection type. If
    /// an alias name is not provided, no alias is created. Note that setting an alias also
    /// requires setting a symbol name.
    #[must_use]
    pub fn alias_name(mut self, alias_name: &str) -> Self {
        self.alias_name = Some(alias_name.to_string());
        self
    }

    /// Specifies whether the keys in the collection are `const`.
    #[must_use]
    pub const fn const_keys(mut self, const_keys: bool) -> Self {
        self.const_keys = const_keys;
        self
    }

    /// Specifies whether the values in the collection are `const`.
    #[must_use]
    pub const fn const_values(mut self, const_values: bool) -> Self {
        self.const_values = const_values;
        self
    }

    /// Specifies whether the visibility of the generated symbol.
    ///
    /// The default is [`Visibility::Inherited`]. Note that this value is only used when
    /// a symbol name is provided. It has no effect otherwise.
    #[must_use]
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Specifies whether the generated collection is a static instance.
    #[must_use]
    pub const fn static_instance(mut self, static_instance: bool) -> Self {
        self.is_static = static_instance;
        self
    }

    /// Specifies whether the generated collection is mutable.
    #[must_use]
    pub const fn mutable(mut self, mutable: bool) -> Self {
        self.is_mutable = mutable;
        self
    }

    /// Emits a frozen hash collection.
    ///
    /// If the emitter's value type has been set, this emits a map. Otherwise, it emits a set.
    ///
    /// # Errors
    ///
    /// This function fails if the emitter was misconfigured.
    pub fn emit_hash_collection<K>(
        &self,
        mut entries: Vec<CollectionEntry<K>>,
    ) -> Result<TokenStream, String>
    where
        K: core::hash::Hash + Eq,
    {
        let hasher = RandomState::default();
        crate::utils::dedup_by_hash_keep_last(
            &mut entries,
            |x| hasher.hash_one(&x.key),
            |x, y| x.key == y.key,
        );

        self.clean_values(&mut entries);

        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_inline_scan(entries)
        } else {
            gen.gen_inline_hash_with_bridge(entries)
        };

        self.postflight(output)
    }

    /// Emits a frozen ordered collection.
    ///
    /// If the emitter's value type has been set, this emits a map. Otherwise, it emits a set.
    ///
    /// # Errors
    ///
    /// This function fails if the emitter was misconfigured.
    #[cfg(feature = "emit")]
    pub fn emit_ordered_collection<K>(
        &self,
        mut entries: Vec<CollectionEntry<K>>,
    ) -> Result<TokenStream, String>
    where
        K: Ord,
    {
        entries.sort_by(|x, y| x.key.cmp(&y.key));
        dedup_by_keep_last(&mut entries, |x, y| x.key == y.key);
        self.clean_values(&mut entries);

        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_inline_scan(entries)
        } else if entries.len() < ORDERED_SCAN_THRESHOLD {
            gen.gen_inline_ordered_scan(entries)
        } else if entries.len() < BINARY_SEARCH_THRESHOLD {
            gen.gen_inline_binary_search(entries)
        } else {
            gen.gen_inline_eytzinger_search(entries)
        };

        self.postflight(output)
    }

    /// Emits a frozen scalar collection.
    ///
    /// If the emitter's value type has been set, this emits a map. Otherwise, it emits a set.
    ///
    /// # Errors
    ///
    /// This function fails if the emitter was misconfigured.
    pub fn emit_scalar_collection<K>(
        &self,
        mut entries: Vec<CollectionEntry<K>>,
    ) -> Result<TokenStream, String>
    where
        K: Scalar,
    {
        entries.sort_by(|x, y| x.key.cmp(&y.key));
        dedup_by_keep_last(&mut entries, |x, y| x.key == y.key);

        self.clean_values(&mut entries);

        let analysis = analyze_scalar_keys(entries.iter().map(|x| x.key));

        let gen = self.preflight(entries.len())?;
        let output = match analysis {
            ScalarKeyAnalysisResult::DenseRange => gen.gen_inline_dense_scalar_lookup(entries),
            ScalarKeyAnalysisResult::SparseRange => gen.gen_inline_sparse_scalar_lookup(entries),
            ScalarKeyAnalysisResult::General => {
                if entries.len() < SCAN_THRESHOLD {
                    gen.gen_inline_scan(entries)
                } else if entries.len() < ORDERED_SCAN_THRESHOLD {
                    gen.gen_inline_ordered_scan(entries)
                } else {
                    gen.gen_inline_hash_with_passthrough(entries, &PassthroughHasher::new())
                }
            }
        };

        self.postflight(output)
    }

    /// Emits a frozen string collection.
    ///
    /// If the emitter's value type has been set, this emits a map. Otherwise, it emits a set.
    ///
    /// # Errors
    ///
    /// This function fails if the emitter was misconfigured.
    pub fn emit_string_collection(
        self,
        mut entries: Vec<CollectionEntry<String>>,
    ) -> Result<TokenStream, String> {
        entries.sort_by(|x, y| x.key.cmp(&y.key));
        dedup_by_keep_last(&mut entries, |x, y| x.key == y.key);

        self.clean_values(&mut entries);

        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_inline_scan(entries)
        } else if entries.len() < ORDERED_SCAN_THRESHOLD {
            gen.gen_inline_ordered_scan(entries)
        } else {
            let iter = entries.iter().map(|x| x.key.as_bytes());

            let bh =
                ahash::RandomState::with_seeds(gen.seeds.0, gen.seeds.1, gen.seeds.2, gen.seeds.3);
            let analysis = analyze_slice_keys(iter, &bh);

            match analysis {
                SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                    let hasher = LeftRangeHasher::new(bh, range.clone());
                    gen.gen_inline_hash_with_range(
                        entries,
                        range,
                        &quote!(InlineLeftRangeHasher),
                        &hasher,
                    )
                }

                SliceKeyAnalysisResult::RightHandSubslice(range) => {
                    let hasher = RightRangeHasher::new(bh, range.clone());
                    gen.gen_inline_hash_with_range(
                        entries,
                        range,
                        &quote!(InlineRightRangeHasher),
                        &hasher,
                    )
                }

                SliceKeyAnalysisResult::Length => {
                    gen.gen_inline_hash_with_passthrough(entries, &PassthroughHasher::new())
                }

                SliceKeyAnalysisResult::General => gen.gen_inline_hash_with_bridge(entries),
            }
        };

        self.postflight(output)
    }

    #[cfg(feature = "macros")]
    pub(crate) fn emit_hash_collection_expr(
        self,
        entries: Vec<CollectionEntry<NonLiteralKey>>,
    ) -> Result<TokenStream, String> {
        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_scan(entries)
        } else {
            gen.gen_hash_with_bridge(entries)
        };

        self.postflight(output)
    }

    #[cfg(feature = "macros")]
    pub(crate) fn emit_ordered_collection_expr(
        self,
        entries: Vec<CollectionEntry<NonLiteralKey>>,
    ) -> Result<TokenStream, String> {
        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_scan(entries)
        } else if entries.len() < ORDERED_SCAN_THRESHOLD {
            gen.gen_ordered_scan(entries)
        } else if entries.len() < BINARY_SEARCH_THRESHOLD {
            gen.gen_binary_search(entries)
        } else {
            gen.gen_eytzinger_search(entries)
        };

        self.postflight(output)
    }

    #[cfg(feature = "macros")]
    pub(crate) fn emit_scalar_collection_expr(
        self,
        entries: Vec<CollectionEntry<NonLiteralKey>>,
    ) -> Result<TokenStream, String> {
        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_scan(entries)
        } else {
            gen.gen_facade_scalar(entries)
        };

        self.postflight(output)
    }

    #[cfg(feature = "macros")]
    pub(crate) fn emit_string_collection_expr(
        self,
        entries: Vec<CollectionEntry<NonLiteralKey>>,
    ) -> Result<TokenStream, String> {
        let gen = self.preflight(entries.len())?;
        let output = if entries.len() < SCAN_THRESHOLD {
            gen.gen_scan(entries)
        } else {
            gen.gen_facade_string(entries)
        };

        self.postflight(output)
    }

    fn clean_values<K>(&self, entries: &mut [CollectionEntry<K>]) {
        if self.value_type.is_none() {
            for e in entries.iter_mut() {
                e.value_expr = parse_quote! { () };
            }
        }
    }

    fn preflight(&self, len: usize) -> Result<Generator, String> {
        if self.is_static && self.is_mutable {
            Err("mutable is not allowed for static collections".to_string())
        } else if self.is_static && self.symbol_name.is_none() {
            Err("symbol_name is required for static collections".to_string())
        } else if self.is_mutable && self.symbol_name.is_none() {
            Err("symbol_name is required for mutable collections".to_string())
        } else if self.alias_name.is_some() && self.symbol_name.is_none() {
            Err("alias_name cannot be used without symbol_name".to_string())
        } else {
            Ok(Generator::new(
                &self.key_type,
                self.value_type.as_ref(),
                len,
            ))
        }
    }

    #[allow(clippy::option_if_let_else)]
    fn postflight(&self, output: Output) -> Result<TokenStream, String> {
        let type_sig = output.type_sig;
        let ctor = output.ctor;
        let visibility = &self.visibility;

        Ok(if self.is_static {
            let symbol_name = format_ident!("{}", self.symbol_name.as_ref().unwrap());
            if self.const_keys && self.const_values {
                if let Some(alias_name) = self.alias_name.as_ref() {
                    let alias_name = format_ident!("{}", alias_name);
                    quote!(
                        #visibility type #alias_name = #type_sig;
                        #visibility static #symbol_name: #alias_name = #ctor;
                    )
                } else {
                    quote!(
                        #visibility static #symbol_name: #type_sig = #ctor;
                    )
                }
            } else if let Some(alias_name) = self.alias_name.as_ref() {
                let alias_name = format_ident!("{}", alias_name);
                quote!(
                    #visibility type #alias_name = #type_sig;
                    #visibility static #symbol_name: std::sync::LazyLock<#alias_name> = std::sync::LazyLock::new(|| { #ctor });
                )
            } else {
                quote!(
                   #visibility static #symbol_name: std::sync::LazyLock<#type_sig> = std::sync::LazyLock::new(|| { #ctor });
                )
            }
        } else if let Some(symbol_name) = self.symbol_name.as_ref() {
            let symbol_name = format_ident!("{}", symbol_name);
            let mutable = if self.is_mutable {
                quote!(mut)
            } else {
                quote!()
            };

            if let Some(alias_name) = self.alias_name.as_ref() {
                let alias_name = format_ident!("{}", alias_name);
                quote!(
                    type #alias_name = #type_sig;
                    let #mutable #symbol_name: #alias_name = #ctor;
                )
            } else {
                quote!(
                    let #mutable #symbol_name: #type_sig = #ctor;
                )
            }
        } else {
            ctor
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_preflight_static_and_mutable() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 })
            .static_instance(true)
            .mutable(true);
        let result = emitter.preflight(10);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "mutable is not allowed for static collections"
        );
    }

    #[test]
    fn test_preflight_static_without_symbol_name() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 }).static_instance(true);
        let result = emitter.preflight(10);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "symbol_name is required for static collections"
        );
    }

    #[test]
    fn test_preflight_mutable_without_symbol_name() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 }).mutable(true);
        let result = emitter.preflight(10);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "symbol_name is required for mutable collections"
        );
    }

    #[test]
    fn test_preflight_alias_without_symbol_name() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 }).alias_name("Alias");
        let result = emitter.preflight(10);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "alias_name cannot be used without symbol_name"
        );
    }

    #[test]
    fn test_preflight_valid_static() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 })
            .symbol_name("SYMBOL")
            .static_instance(true);
        let result = emitter.preflight(10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preflight_valid_mutable() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 })
            .symbol_name("SYMBOL")
            .mutable(true);
        let result = emitter.preflight(10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preflight_valid_alias() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 })
            .symbol_name("SYMBOL")
            .alias_name("Alias");
        let result = emitter.preflight(10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preflight_valid() {
        let emitter = CollectionEmitter::new(&parse_quote! { i32 });
        let result = emitter.preflight(10);
        assert!(result.is_ok());
    }
}
