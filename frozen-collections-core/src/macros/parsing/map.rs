use crate::macros::parsing::long_form_map::LongFormMap;
use crate::macros::parsing::short_form_map::ShortFormMap;
use syn::parse::Parse;
use syn::{Token, Visibility};

pub enum Map {
    Short(ShortFormMap),
    Long(LongFormMap),
}

impl Parse for Map {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<Visibility>()?;
        if visibility != Visibility::Inherited && !input.peek(Token![static]) {
            return Err(input.error("expected `static`"));
        }

        if input.peek(Token![static]) {
            input.parse::<Token![static]>()?;
            let mut m = input.parse::<LongFormMap>()?;
            m.visibility = visibility;
            m.is_static = true;
            Ok(Self::Long(m))
        } else if input.peek(Token![let]) {
            input.parse::<Token![let]>()?;

            let is_mutable = if input.peek(Token![mut]) {
                input.parse::<Token![mut]>()?;
                true
            } else {
                false
            };

            let mut m = input.parse::<LongFormMap>()?;
            m.visibility = visibility;
            m.is_static = false;
            m.is_mutable = is_mutable;
            Ok(Self::Long(m))
        } else {
            Ok(Self::Short(input.parse()?))
        }
    }
}
