use crate::macros::parsing::long_form_set::LongFormSet;
use crate::macros::parsing::short_form_set::ShortFormSet;
use syn::parse::Parse;
use syn::{Token, Visibility};

pub enum Set {
    Short(ShortFormSet),
    Long(LongFormSet),
}

impl Parse for Set {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<Visibility>()?;
        if visibility != Visibility::Inherited && !input.peek(syn::Token![static]) {
            return Err(input.error("expected `static`"));
        }

        if input.peek(Token![static]) {
            input.parse::<Token![static]>()?;
            let mut s = input.parse::<LongFormSet>()?;
            s.visibility = visibility;
            s.is_static = true;
            Ok(Self::Long(s))
        } else if input.peek(Token![let]) {
            input.parse::<Token![let]>()?;

            let is_mutable = if input.peek(Token![mut]) {
                input.parse::<Token![mut]>()?;
                true
            } else {
                false
            };

            let mut s = input.parse::<LongFormSet>()?;
            s.visibility = visibility;
            s.is_static = false;
            s.is_mutable = is_mutable;
            Ok(Self::Long(s))
        } else {
            Ok(Self::Short(input.parse()?))
        }
    }
}
