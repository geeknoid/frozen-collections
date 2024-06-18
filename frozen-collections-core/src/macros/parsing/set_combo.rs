use crate::macros::parsing::set::Set;
use crate::macros::parsing::static_set::StaticSet;
use syn::parse::Parse;
use syn::Visibility;

pub enum SetCombo {
    Set(Set),
    StaticSet(StaticSet),
}

impl Parse for SetCombo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<Visibility>()?;
        if visibility != Visibility::Inherited && !input.peek(syn::Token![static]) {
            return Err(input.error("expected `static`"));
        }

        if input.peek(syn::Token![static]) {
            let mut s = input.parse::<StaticSet>()?;
            s.visibility = visibility;
            Ok(Self::StaticSet(s))
        } else {
            Ok(Self::Set(input.parse()?))
        }
    }
}
