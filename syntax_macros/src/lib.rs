#![no_std]
#![feature(allocator_api)]
#![feature(proc_macro_diagnostic)]

extern crate alloc;

use {
    alloc::{format, string::ToString},
    core::{iter::Peekable, str::FromStr},
    proc_macro::{Span, TokenStream, TokenTree},
};

#[proc_macro]
pub fn syntax(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut input = input.split_whitespace();
    let mut result = stringify!("");

    TokenStream::from_str(result).unwrap()
}

// macro_rules! expect_token {
//     ($to_match: ident, $variant: ident, $value: literal, $error: literal) => {
//         expect_token!(@_internal $to_match, next, $variant, token.span().error($error).emit(), expect_token!(@_match_value $value, $error))
//     };
//     ($to_match: ident, $variant: ident, $error: literal) => {
//         expect_token!(@_internal $to_match, next, $variant, token.span().error($error).emit(), Some(token))
//     };
//     (@peek $to_match: ident, $variant: ident, $value: literal) => {
//         expect_token!(@_internal $to_match, peek, $variant, (), expect_token!(@_match_value $value, ()))
//     };
//
//     (@_match_value $value: literal, $error: stmt) => {
//         if token.to_string() == *$value {
//             Some(token)
//         } else {
//             $error
//             None
//         }
//     };
//     (@_internal $to_match: ident, $method: ident, $variant: ident, $error: stmt, $($value_fn: tt)*) => {
//         if let Some(token) = $to_match.$method() {
//             if let TokenTree::$variant(token) = token {
//                 $($value_fn)*
//             } else {
//                 $error
//                 None
//             }
//         } else {
//             None
//         }
//     };
// }

// #[proc_macro_attribute]
// pub fn syntax(attrs: TokenStream, input: TokenStream) -> TokenStream {}
//
// enum MacroSection {
//     Syntax(Span, TokenStream),
//     Handler(Span, TokenStream),
// }

// fn next_section(input: Peekable<impl Iterator<Item = TokenTree>>) -> MacroSection {
//     let hash = expect_token!(
//         input,
//         Punct,
//         "#",
//         "Expected attribute (either `#[syntax]` or `#[handler]`)"
//     )
//     .unwrap();
//     let attr = expect_token!(
//         input,
//         Group,
//         "Expected attribute (either `#[syntax]` or `#[handler]`)"
//     )
//     .unwrap();
//
//     let mut stream = TokenStream::new();
//     while input.peek().is_some() {
//         if expect_token!(@peek input, Punct, "#") {
//             return MacroSection::Syntax(Span::call_site(), stream);
//         }
//         stream.extend_one(input.next().unwrap());
//     }
// }
