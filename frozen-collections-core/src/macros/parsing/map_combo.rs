use crate::macros::parsing::map::Map;
use crate::macros::parsing::static_map::StaticMap;
use syn::parse::Parse;
use syn::Visibility;

pub enum MapCombo {
    Map(Map),
    StaticMap(StaticMap),
}

impl Parse for MapCombo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<Visibility>()?;
        if visibility != Visibility::Inherited && !input.peek(syn::Token![static]) {
            return Err(input.error("expected `static`"));
        }

        if input.peek(syn::Token![static]) {
            let mut m = input.parse::<StaticMap>()?;
            m.visibility = visibility;
            Ok(Self::StaticMap(m))
        } else {
            Ok(Self::Map(input.parse()?))
        }
    }
}
