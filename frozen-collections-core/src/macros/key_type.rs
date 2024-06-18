use syn::{Expr, Lit};

pub enum KeyType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    String,
    Complex,
}

pub fn extract(expr: Expr) -> KeyType {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(_) => KeyType::String,
            Lit::Int(s) => match s.suffix() {
                "i8" => KeyType::I8,
                "i16" => KeyType::I16,
                "i32" => KeyType::I32,
                "i64" => KeyType::I64,
                "i128" => KeyType::I128,
                "u8" => KeyType::U8,
                "u16" => KeyType::U16,
                "u64" => KeyType::U64,
                "u128" => KeyType::U128,
                _ => KeyType::U32,
            },
            _ => KeyType::Complex,
        },
        _ => KeyType::Complex,
    }
}
