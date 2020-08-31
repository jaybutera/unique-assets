// Macro to make a struct implement Nft
// Implement PartialEq on id as well
// #![feature(trace_macros)]

mod kw {
    syn::custom_keyword!(Unique),
    syn::custom_keyword!(Mintable),
    syn::custom_keyword!(Burnable),
}

enum AssetFeature {
    Unique(UniqueType),
    Mintable(MintableType),
}

impl syn::parse::Parse for AssetFeature {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        if input.peek(kw::Unique) {
        }
    }
}

macro_rules! features_of {
    ( $module:ident,
      [ $($trait:ident),+ ]
      where,
      $($type_decls:expr),* ) =>
    {
    };
}

fn main() {
    features_of! {
        Module,
        [UniqueAssets, Mintable]
        where,
            AccountId = u8
    };
}
