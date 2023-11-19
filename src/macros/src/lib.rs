#![no_std]
extern crate alloc;

mod deserialize;

pub(crate) mod _crate_prelude {
    pub use {
        alloc::{format, string::ToString, vec, vec::Vec},
        core::str::FromStr,
        proc_macro::{
            Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
        },
    };
}
use _crate_prelude::*;

macro_rules! err {
    // Creates an error across the whole macro invocation.
    ($err:literal) => {{
        let start = Span::call_site();
        let end = Span::call_site();

        err!(start, end, $err)
    }};

    // Creates an error at a single token's location.
    (@token $token:ident, $err:literal) => {{
        let span = $token.span();

        err!(span, span, $err)
    }};

    // Creates an error at a tokenstream's location.
    (@stream $stream:ident, $err:literal) => {{
        let mut stream = $stream.into_iter();
        let start = stream.next().unwrap().span();
        let end = if let Some(last) = stream.last() {
            last.span()
        } else {
            start
        };

        err!(start, end, $err)
    }};

    // Creates an error from a span at $start to a span at $end.
    ($start:ident, $end:ident, $err:literal) => {{
        // https://github.com/dtolnay/syn/blob/e20379473b266ec5904bc87415d80de3ab1ac059/src/error.rs#L275
        // Once proc_macro_diagnostics is stabilised, we won't have to go through this pain, and can
        // emit errors directly from tokens. I think.
        TokenStream::from_iter(vec![
            TokenTree::Punct({
                let mut punct = Punct::new(':', Spacing::Joint);
                punct.set_span($start);
                punct
            }),
            TokenTree::Punct({
                let mut punct = Punct::new(':', Spacing::Alone);
                punct.set_span($start);
                punct
            }),
            TokenTree::Ident(Ident::new("core", $start)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("compile_error", $start)),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group({
                let mut group = Group::new(
                    Delimiter::Brace,
                    TokenStream::from_iter(vec![TokenTree::Literal(Literal::string($err))]),
                );
                group.set_span($end);
                group
            }),
        ])
    }};
}
pub(crate) use err;

#[proc_macro_attribute]
pub fn deserialize_by(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut attr = attr.into_iter();
    let Some(deserialization_strategy) = attr.next() else {
        return err!("Please specify a deserialization strategy.");
    };

    match deserialization_strategy.to_string().as_str() {
        "fill" => deserialize::fill::fill(input),
        _ => err!(@token
            deserialization_strategy,
            "Unknown deserialization strategy. Please look at the docs for available strategies."
        ),
    }
}
