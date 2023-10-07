use super::*;

pub trait Tokenizer<InputType>: Sized {
    fn tokenize(input: &Cursor<InputType>) -> Self;
}

#[derive(Default)]
pub struct TokenParser<InputType, TokenType: Tokenizer<InputType>> {
    _input: PhantomData<InputType>,
    _token: PhantomData<TokenType>,
}
impl<InputType, TokenType: Tokenizer<InputType>> TokenParser<InputType, TokenType> {
    pub fn parse(input: Vec<InputType>) -> Vec<TokenType> {
        let mut input = Cursor { input, index: 0 };
        let mut tokens = Vec::new();

        while input.peek_next_value().is_some() {
            tokens.push(TokenType::tokenize(&input))
        }

        tokens
    }
}
