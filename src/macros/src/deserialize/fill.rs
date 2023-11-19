use {crate::_crate_prelude::*, syntax_core::prelude::*};

pub fn fill(input: TokenStream) -> TokenStream {
    let struct_definition = input.clone();
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut cursor = Cursor::new(tokens).unwrap();

    TokenStream::from_str(&format!(
        "
        {struct_definition}
        impl syntax::prelude::Deserialize for
        "
    ))
    .unwrap()
}
