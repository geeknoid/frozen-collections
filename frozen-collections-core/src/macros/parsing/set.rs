use crate::macros::parsing::long_form_set::LongFormSet;
use crate::macros::parsing::short_form_set::ShortFormSet;
use syn::parse::Parse;
use syn::{Token, Visibility};

#[expect(clippy::large_enum_variant, reason = "Large is in the eye of the beholder, this is just fine")]
pub enum Set {
    Short(ShortFormSet),
    Long(LongFormSet),
}

impl Parse for Set {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<Visibility>()?;
        if visibility != Visibility::Inherited && !input.peek(Token![static]) {
            return Err(input.error("expected `static`"));
        }

        if input.peek(Token![static]) {
            _ = input.parse::<Token![static]>()?;
            let mut s = input.parse::<LongFormSet>()?;
            s.visibility = visibility;
            s.is_static = true;
            Ok(Self::Long(s))
        } else if input.peek(Token![let]) {
            _ = input.parse::<Token![let]>()?;

            let is_mutable = if input.peek(Token![mut]) {
                _ = input.parse::<Token![mut]>()?;
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
