use crate::emit::{CollectionEmitter, CollectionEntry, NonLiteralKey};
use crate::macros::parsing::entry::Entry;
use crate::macros::parsing::payload::Payload;
use crate::traits::Scalar;
use alloc::format;
use core::fmt::Display;
use core::str::FromStr;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Expr, ExprLit, Lit, LitInt, LitStr, parse_str, parse2};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum MacroKind {
    Scalar,
    String,
    Hashed,
    Ordered,
}

#[derive(Eq, PartialEq)]
enum ScalarType {
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
    Undecided,
}

#[derive(Eq, PartialEq)]
enum DiscoveredKeyKind {
    LiteralScalar(ScalarType),
    LiteralString,
    Expression,
}

enum EffectiveKeyKind {
    AllLiteralScalars(ScalarType),
    LiteralAndExpressionScalars,
    AllLiteralStrings,
    LiteralAndExpressionStrings,
    Hashed,
    Ordered,
}

pub(super) fn process(payload: Payload, emitter: CollectionEmitter, macro_kind: MacroKind) -> syn::Result<TokenStream> {
    let entries = payload.entries;
    if entries.is_empty() {
        return if emitter.inferred_key_type {
            Err(syn::Error::new(Span::call_site(), "no collection entries supplied"))
        } else {
            emitter
                .const_keys(true)
                .const_values(true)
                .emit_hash_collection(Vec::<CollectionEntry<i32>>::new())
                .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
        };
    }

    match assess_keys(&entries, macro_kind)? {
        EffectiveKeyKind::AllLiteralScalars(ScalarType::I8) => handle_literal_scalar_keys::<i8>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::I16) => handle_literal_scalar_keys::<i16>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::I32) => handle_literal_scalar_keys::<i32>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::I64) => handle_literal_scalar_keys::<i64>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::ISize) => handle_literal_scalar_keys::<isize>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::U8) => handle_literal_scalar_keys::<u8>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::U16) => handle_literal_scalar_keys::<u16>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::U32) => handle_literal_scalar_keys::<u32>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::U64) => handle_literal_scalar_keys::<u64>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::USize) => handle_literal_scalar_keys::<usize>(emitter, entries, ""),
        EffectiveKeyKind::AllLiteralScalars(ScalarType::Undecided) => handle_literal_scalar_keys::<i32>(emitter, entries, "i32"),
        EffectiveKeyKind::LiteralAndExpressionScalars => handle_non_literal_scalar_keys(emitter, entries),
        EffectiveKeyKind::AllLiteralStrings => handle_literal_string_keys(emitter, entries),
        EffectiveKeyKind::LiteralAndExpressionStrings => handle_non_literal_string_keys(emitter, entries),
        EffectiveKeyKind::Hashed => handle_hashed_keys(emitter, entries),
        EffectiveKeyKind::Ordered => handle_ordered_keys(emitter, entries),
    }
}

fn assess_keys(entries: &[Entry], macro_kind: MacroKind) -> syn::Result<EffectiveKeyKind> {
    let mut num_strings = 0;
    let mut num_scalars = 0;
    let mut scalar_type: ScalarType = ScalarType::Undecided;

    for entry in entries {
        let discovered_key_kind = match &entry.key {
            Expr::Lit(expr) => eval_literal_expr(expr)?,
            Expr::Group(group) => match &*group.expr {
                Expr::Lit(expr) => eval_literal_expr(expr)?,
                _ => DiscoveredKeyKind::Expression,
            },
            _ => DiscoveredKeyKind::Expression,
        };

        if macro_kind == MacroKind::Scalar && discovered_key_kind == DiscoveredKeyKind::LiteralString {
            return Err(syn::Error::new(Span::call_site(), "scalar macro cannot contain string keys"));
        } else if macro_kind == MacroKind::String
            && discovered_key_kind != DiscoveredKeyKind::LiteralString
            && discovered_key_kind != DiscoveredKeyKind::Expression
        {
            return Err(syn::Error::new(Span::call_site(), "string macro cannot contain scalar keys"));
        }

        match discovered_key_kind {
            DiscoveredKeyKind::LiteralScalar(ScalarType::Undecided) => num_scalars += 1,
            DiscoveredKeyKind::LiteralScalar(discovered_scalar_type) => {
                num_scalars += 1;
                if scalar_type == ScalarType::Undecided {
                    scalar_type = discovered_scalar_type;
                } else if discovered_scalar_type != scalar_type {
                    return Err(syn::Error::new(Span::call_site(), "incompatible scalar literal type"));
                }
            }

            DiscoveredKeyKind::LiteralString => num_strings += 1,
            DiscoveredKeyKind::Expression => {}
        }
    }

    Ok(if num_scalars == entries.len() {
        EffectiveKeyKind::AllLiteralScalars(scalar_type)
    } else if num_scalars > 0 && num_strings == 0 {
        EffectiveKeyKind::LiteralAndExpressionScalars
    } else if num_strings == entries.len() {
        EffectiveKeyKind::AllLiteralStrings
    } else if num_strings > 0 {
        EffectiveKeyKind::LiteralAndExpressionStrings
    } else {
        match macro_kind {
            MacroKind::Scalar => EffectiveKeyKind::LiteralAndExpressionScalars,
            MacroKind::String => EffectiveKeyKind::LiteralAndExpressionStrings,
            MacroKind::Hashed => EffectiveKeyKind::Hashed,
            MacroKind::Ordered => EffectiveKeyKind::Ordered,
        }
    })
}

fn eval_literal_expr(expr: &ExprLit) -> syn::Result<DiscoveredKeyKind> {
    let kind = match &expr.lit {
        Lit::Str(_) => DiscoveredKeyKind::LiteralString,
        Lit::Int(expr) => match expr.suffix() {
            "i8" => DiscoveredKeyKind::LiteralScalar(ScalarType::I8),
            "i16" => DiscoveredKeyKind::LiteralScalar(ScalarType::I16),
            "i32" => DiscoveredKeyKind::LiteralScalar(ScalarType::I32),
            "i64" => DiscoveredKeyKind::LiteralScalar(ScalarType::I64),
            "isize" => DiscoveredKeyKind::LiteralScalar(ScalarType::ISize),
            "u8" => DiscoveredKeyKind::LiteralScalar(ScalarType::U8),
            "u16" => DiscoveredKeyKind::LiteralScalar(ScalarType::U16),
            "u32" => DiscoveredKeyKind::LiteralScalar(ScalarType::U32),
            "u64" => DiscoveredKeyKind::LiteralScalar(ScalarType::U64),
            "usize" => DiscoveredKeyKind::LiteralScalar(ScalarType::USize),
            "" => DiscoveredKeyKind::LiteralScalar(ScalarType::Undecided),
            _ => {
                return Err(syn::Error::new_spanned(
                    expr,
                    format!("unknown suffix {} for scalar value", expr.suffix()),
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(expr, "invalid literal, expecting a scalar or string value"));
        }
    };

    Ok(kind)
}

fn handle_literal_scalar_keys<K>(emitter: CollectionEmitter, entries: Vec<Entry>, suffix: &str) -> syn::Result<TokenStream>
where
    K: Scalar + Ord + FromStr,
    K::Err: Display,
{
    let mut coll_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        let lit = parse2::<LitInt>(entry.key.to_token_stream())?;
        let k = lit.base10_parse::<K>()?;

        let key = if suffix.is_empty() {
            entry.key
        } else {
            parse_str::<Expr>(&format!("{lit}{suffix}"))?
        };

        if entry.value.is_some() {
            coll_entries.push(CollectionEntry::map_entry(k, key, entry.value.unwrap()));
        } else {
            coll_entries.push(CollectionEntry::set_entry(k, key));
        }
    }

    emitter
        .const_keys(true)
        .const_values(true)
        .emit_scalar_collection(coll_entries)
        .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
}

fn handle_literal_string_keys(emitter: CollectionEmitter, entries: Vec<Entry>) -> syn::Result<TokenStream> {
    let mut coll_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        let ls = parse2::<LitStr>(entry.key.to_token_stream())?;

        if entry.value.is_some() {
            coll_entries.push(CollectionEntry::map_entry(ls.value(), entry.key, entry.value.unwrap()));
        } else {
            coll_entries.push(CollectionEntry::set_entry(ls.value(), entry.key));
        }
    }

    emitter
        .const_keys(true)
        .const_values(true)
        .emit_string_collection(coll_entries)
        .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
}

fn handle_non_literal_scalar_keys(emitter: CollectionEmitter, entries: Vec<Entry>) -> syn::Result<TokenStream> {
    let mut coll_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        if entry.value.is_some() {
            coll_entries.push(CollectionEntry::map_entry(NonLiteralKey {}, entry.key, entry.value.unwrap()));
        } else {
            coll_entries.push(CollectionEntry::set_entry(NonLiteralKey {}, entry.key));
        }
    }

    emitter
        .const_keys(false)
        .const_values(false)
        .emit_scalar_collection_expr(coll_entries)
        .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
}

fn handle_non_literal_string_keys(emitter: CollectionEmitter, entries: Vec<Entry>) -> syn::Result<TokenStream> {
    let mut coll_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        if entry.value.is_some() {
            coll_entries.push(CollectionEntry::map_entry(NonLiteralKey {}, entry.key, entry.value.unwrap()));
        } else {
            coll_entries.push(CollectionEntry::set_entry(NonLiteralKey {}, entry.key));
        }
    }

    emitter
        .const_keys(false)
        .const_values(false)
        .emit_string_collection_expr(coll_entries)
        .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
}

fn handle_hashed_keys(emitter: CollectionEmitter, entries: Vec<Entry>) -> syn::Result<TokenStream> {
    let mut coll_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        if entry.value.is_some() {
            coll_entries.push(CollectionEntry::map_entry(NonLiteralKey {}, entry.key, entry.value.unwrap()));
        } else {
            coll_entries.push(CollectionEntry::set_entry(NonLiteralKey {}, entry.key));
        }
    }

    emitter
        .const_keys(false)
        .const_values(false)
        .emit_hash_collection_expr(coll_entries)
        .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
}

fn handle_ordered_keys(emitter: CollectionEmitter, entries: Vec<Entry>) -> syn::Result<TokenStream> {
    let mut coll_entries = Vec::with_capacity(entries.len());
    for entry in entries {
        if entry.value.is_some() {
            coll_entries.push(CollectionEntry::map_entry(NonLiteralKey {}, entry.key, entry.value.unwrap()));
        } else {
            coll_entries.push(CollectionEntry::set_entry(NonLiteralKey {}, entry.key));
        }
    }

    emitter
        .const_keys(false)
        .const_values(false)
        .emit_ordered_collection_expr(coll_entries)
        .map_err(|e| syn::Error::new(Span::call_site(), e.as_str()))
}
