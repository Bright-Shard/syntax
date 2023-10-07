use std::{iter::Peekable, str::Chars};

pub struct Parser<InputType> {
    pub patterns: Vec<Pattern<InputType>>,
}
impl<InputType> Parser<InputType> {
    pub fn parse(&self, input: InputType) {}
}

pub struct Pattern<ParseTy> {
    components: Vec<PatternType<ParseTy>>,
}

pub enum PatternType<ParseTy> {
    Absolute(ParseTy),
    Variable,
}
impl<ParseTy: Iterator> PatternType<ParseTy> {
    pub fn try_match(&self, char_buffer: Peekable<ParseTy>) {}
}

// vec![Pattern::Absolute(u8"bplist00"), Pattern::Repeated(Pattern::Variable)]
// u8"bplist00" REPEATED(VARIABLE)

#[cfg(test)]
mod tests {
    use super::*;
    use syntax_macros::syntax;

    #[test]
    fn syntax_macro() {
        assert_eq!("lit", syntax!("hi"));
        assert_eq!("lit", syntax!(b"hi"));
        assert_eq!("lit", syntax!(2));
        assert_eq!("repeated", syntax!(REPEATED("hi")));
    }
}
