#![cfg(feature = "emit")]
#![allow(missing_docs, reason = "test module")]

use frozen_collections::emit::{CollectionEmitter, CollectionEntry};
use syn::parse_quote;

fn str_entries() -> Vec<CollectionEntry<String>> {
    vec![
        CollectionEntry::map_entry("Hello".to_string(), parse_quote! { "Hello" }, parse_quote! { 1 }),
        CollectionEntry::map_entry("World".to_string(), parse_quote! { "World" }, parse_quote! { 2 }),
        CollectionEntry::map_entry("Rust".to_string(), parse_quote! { "Rust" }, parse_quote! { 3 }),
        CollectionEntry::map_entry("Frozen".to_string(), parse_quote! { "Frozen" }, parse_quote! { 4 }),
        CollectionEntry::map_entry("Collections".to_string(), parse_quote! { "Collections" }, parse_quote! { 5 }),
    ]
}

fn hash_entries() -> Vec<CollectionEntry<&'static str>> {
    vec![
        CollectionEntry::map_entry("Hello", parse_quote! { "Hello" }, parse_quote! { 1 }),
        CollectionEntry::map_entry("World", parse_quote! { "World" }, parse_quote! { 2 }),
        CollectionEntry::map_entry("Rust", parse_quote! { "Rust" }, parse_quote! { 3 }),
        CollectionEntry::map_entry("Frozen", parse_quote! { "Frozen" }, parse_quote! { 4 }),
        CollectionEntry::map_entry("Collections", parse_quote! { "Collections" }, parse_quote! { 5 }),
    ]
}

fn small_hash_entries() -> Vec<CollectionEntry<&'static str>> {
    vec![
        CollectionEntry::map_entry("A", parse_quote! { "A" }, parse_quote! { 1 }),
        CollectionEntry::map_entry("B", parse_quote! { "B" }, parse_quote! { 2 }),
    ]
}

fn scalar_entries() -> Vec<CollectionEntry<i32>> {
    vec![
        CollectionEntry::map_entry(1, parse_quote! { 1 }, parse_quote! { "one" }),
        CollectionEntry::map_entry(2, parse_quote! { 2 }, parse_quote! { "two" }),
        CollectionEntry::map_entry(3, parse_quote! { 3 }, parse_quote! { "three" }),
        CollectionEntry::map_entry(4, parse_quote! { 4 }, parse_quote! { "four" }),
        CollectionEntry::map_entry(5, parse_quote! { 5 }, parse_quote! { "five" }),
        CollectionEntry::map_entry(6, parse_quote! { 6 }, parse_quote! { "six" }),
        CollectionEntry::map_entry(7, parse_quote! { 7 }, parse_quote! { "seven" }),
        CollectionEntry::map_entry(8, parse_quote! { 8 }, parse_quote! { "eight" }),
        CollectionEntry::map_entry(9, parse_quote! { 9 }, parse_quote! { "nine" }),
        CollectionEntry::map_entry(10, parse_quote! { 10 }, parse_quote! { "ten" }),
    ]
}

fn dense_scalar_entries() -> Vec<CollectionEntry<i32>> {
    vec![
        CollectionEntry::map_entry(0, parse_quote! { 0 }, parse_quote! { "zero" }),
        CollectionEntry::map_entry(1, parse_quote! { 1 }, parse_quote! { "one" }),
        CollectionEntry::map_entry(2, parse_quote! { 2 }, parse_quote! { "two" }),
        CollectionEntry::map_entry(3, parse_quote! { 3 }, parse_quote! { "three" }),
        CollectionEntry::map_entry(4, parse_quote! { 4 }, parse_quote! { "four" }),
    ]
}

fn ordered_entries() -> Vec<CollectionEntry<i32>> {
    vec![
        CollectionEntry::map_entry(10, parse_quote! { 10 }, parse_quote! { "ten" }),
        CollectionEntry::map_entry(20, parse_quote! { 20 }, parse_quote! { "twenty" }),
        CollectionEntry::map_entry(30, parse_quote! { 30 }, parse_quote! { "thirty" }),
        CollectionEntry::map_entry(40, parse_quote! { 40 }, parse_quote! { "forty" }),
        CollectionEntry::map_entry(50, parse_quote! { 50 }, parse_quote! { "fifty" }),
    ]
}

fn set_entries() -> Vec<CollectionEntry<&'static str>> {
    vec![
        CollectionEntry::set_entry("Alpha", parse_quote! { "Alpha" }),
        CollectionEntry::set_entry("Beta", parse_quote! { "Beta" }),
        CollectionEntry::set_entry("Gamma", parse_quote! { "Gamma" }),
        CollectionEntry::set_entry("Delta", parse_quote! { "Delta" }),
        CollectionEntry::set_entry("Epsilon", parse_quote! { "Epsilon" }),
    ]
}

// --- Hash collection tests ---

#[test]
fn emit_hash_map_expression() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(!tokens.is_empty());
}

#[test]
fn emit_hash_map_let_binding() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("my_map")
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("my_map"));
}

#[test]
fn emit_hash_map_let_mut_binding() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("my_map")
        .mutable(true)
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("mut"));
    assert!(tokens.contains("my_map"));
}

#[test]
fn emit_hash_map_static_const() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("MY_MAP")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("MY_MAP"));
    assert!(tokens.contains("static"));
}

#[test]
fn emit_hash_map_static_lazy() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("MY_MAP")
        .static_instance(true)
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("MY_MAP"));
    assert!(tokens.contains("LazyLock"));
}

#[test]
fn emit_hash_map_with_alias() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("MY_MAP")
        .alias_name("MyMap")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("MyMap"));
    assert!(tokens.contains("MY_MAP"));
}

#[test]
fn emit_hash_map_with_alias_lazy() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("MY_MAP")
        .alias_name("MyMap")
        .static_instance(true)
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("MyMap"));
    assert!(tokens.contains("LazyLock"));
}

#[test]
fn emit_hash_map_let_with_alias() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("my_map")
        .alias_name("MyMap")
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("MyMap"));
    assert!(tokens.contains("my_map"));
}

#[test]
fn emit_small_hash_map_uses_scan() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .emit_hash_collection(small_hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("InlineScanMap"));
}

#[test]
fn emit_hash_set() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str }).emit_hash_collection(set_entries());
    let tokens = result.unwrap().to_string();
    assert!(!tokens.is_empty());
}

// --- Scalar collection tests ---

#[test]
fn emit_scalar_map() {
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .symbol_name("SCALAR_MAP")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_scalar_collection(scalar_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("SCALAR_MAP"));
}

#[test]
fn emit_dense_scalar_map() {
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .symbol_name("DENSE_MAP")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_scalar_collection(dense_scalar_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("DENSE_MAP"));
    assert!(tokens.contains("DenseScalarLookup"));
}

#[test]
fn emit_scalar_set() {
    let entries = vec![
        CollectionEntry::set_entry(1, parse_quote! { 1 }),
        CollectionEntry::set_entry(2, parse_quote! { 2 }),
        CollectionEntry::set_entry(3, parse_quote! { 3 }),
        CollectionEntry::set_entry(4, parse_quote! { 4 }),
        CollectionEntry::set_entry(5, parse_quote! { 5 }),
    ];

    let result = CollectionEmitter::new(&parse_quote! { i32 }).emit_scalar_collection(entries);
    let _ = result.unwrap();
}

// --- String collection tests ---

#[test]
fn emit_string_map() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("STRING_MAP")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_string_collection(str_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("STRING_MAP"));
}

#[test]
fn emit_string_set() {
    let entries = vec![
        CollectionEntry::set_entry("Hello".to_string(), parse_quote! { "Hello" }),
        CollectionEntry::set_entry("World".to_string(), parse_quote! { "World" }),
        CollectionEntry::set_entry("Rust".to_string(), parse_quote! { "Rust" }),
        CollectionEntry::set_entry("Frozen".to_string(), parse_quote! { "Frozen" }),
        CollectionEntry::set_entry("Collections".to_string(), parse_quote! { "Collections" }),
    ];

    let result = CollectionEmitter::new(&parse_quote! { &'static str }).emit_string_collection(entries);
    let _ = result.unwrap();
}

// --- Ordered collection tests ---

#[test]
fn emit_ordered_map() {
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .symbol_name("ORDERED_MAP")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_ordered_collection(ordered_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("ORDERED_MAP"));
    assert!(tokens.contains("Eytzinger"));
}

#[test]
fn emit_small_ordered_map_uses_scan() {
    let entries = vec![
        CollectionEntry::map_entry(1, parse_quote! { 1 }, parse_quote! { "one" }),
        CollectionEntry::map_entry(2, parse_quote! { 2 }, parse_quote! { "two" }),
    ];

    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .emit_ordered_collection(entries);
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("InlineScanMap"));
}

#[test]
fn emit_ordered_set() {
    let entries = vec![
        CollectionEntry::set_entry(10, parse_quote! { 10 }),
        CollectionEntry::set_entry(20, parse_quote! { 20 }),
        CollectionEntry::set_entry(30, parse_quote! { 30 }),
        CollectionEntry::set_entry(40, parse_quote! { 40 }),
        CollectionEntry::set_entry(50, parse_quote! { 50 }),
    ];

    let result = CollectionEmitter::new(&parse_quote! { i32 }).emit_ordered_collection(entries);
    let _ = result.unwrap();
}

// --- Empty collection tests ---

#[test]
fn emit_empty_hash_map() {
    let entries: Vec<CollectionEntry<&str>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .emit_hash_collection(entries);
    let _ = result.unwrap();
}

#[test]
fn emit_empty_scalar_map() {
    let entries: Vec<CollectionEntry<i32>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .emit_scalar_collection(entries);
    let _ = result.unwrap();
}

#[test]
fn emit_empty_string_map() {
    let entries: Vec<CollectionEntry<String>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .emit_string_collection(entries);
    let _ = result.unwrap();
}

#[test]
fn emit_empty_ordered_map() {
    let entries: Vec<CollectionEntry<i32>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .emit_ordered_collection(entries);
    let _ = result.unwrap();
}

// --- Single entry tests ---

#[test]
fn emit_single_entry_hash_map() {
    let entries = vec![CollectionEntry::map_entry("only", parse_quote! { "only" }, parse_quote! { 42 })];
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .emit_hash_collection(entries);
    let _ = result.unwrap();
}

#[test]
fn emit_single_entry_scalar_map() {
    let entries = vec![CollectionEntry::map_entry(42, parse_quote! { 42 }, parse_quote! { "answer" })];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { &'static str })
        .emit_scalar_collection(entries);
    let _ = result.unwrap();
}

// --- Duplicate entry tests ---

#[test]
fn emit_hash_map_deduplicates() {
    let entries = vec![
        CollectionEntry::map_entry("A", parse_quote! { "A" }, parse_quote! { 1 }),
        CollectionEntry::map_entry("A", parse_quote! { "A" }, parse_quote! { 2 }),
        CollectionEntry::map_entry("B", parse_quote! { "B" }, parse_quote! { 3 }),
    ];
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .emit_hash_collection(entries);
    let _ = result.unwrap();
}

// --- Error condition tests ---

#[test]
fn emit_fails_static_and_mutable() {
    let entries: Vec<CollectionEntry<i32>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { i32 })
        .symbol_name("BAD")
        .static_instance(true)
        .mutable(true)
        .emit_hash_collection(entries);
    let _ = result.unwrap_err();
}

#[test]
fn emit_fails_static_without_symbol() {
    let entries: Vec<CollectionEntry<i32>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { i32 })
        .static_instance(true)
        .emit_hash_collection(entries);
    let _ = result.unwrap_err();
}

#[test]
fn emit_fails_mutable_without_symbol() {
    let entries: Vec<CollectionEntry<i32>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { i32 })
        .mutable(true)
        .emit_hash_collection(entries);
    let _ = result.unwrap_err();
}

#[test]
fn emit_fails_alias_without_symbol() {
    let entries: Vec<CollectionEntry<i32>> = vec![];
    let result = CollectionEmitter::new(&parse_quote! { i32 })
        .value_type(&parse_quote! { i32 })
        .alias_name("Bad")
        .emit_hash_collection(entries);
    let _ = result.unwrap_err();
}

// --- Visibility tests ---

#[test]
fn emit_pub_static() {
    let result = CollectionEmitter::new(&parse_quote! { &'static str })
        .value_type(&parse_quote! { i32 })
        .symbol_name("PUB_MAP")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .visibility(parse_quote! { pub })
        .emit_hash_collection(hash_entries());
    let tokens = result.unwrap().to_string();
    assert!(tokens.contains("pub"));
    assert!(tokens.contains("PUB_MAP"));
}
