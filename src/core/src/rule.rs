use crate::cursor::Cursor;

pub trait Rule<In, Out, Err> {
    fn process(input: &mut Cursor<In>) -> Result<Out, Err>;
}

#[cfg(test)]
mod tests {
    use crate::cursor::Cursor;

    #[test]
    fn kind_of_json_parser() {
        let input = r#"0{"name": "Bob", "age": 11}"#;
        let mut cursor = Cursor::new(input.chars().collect()).unwrap();
    }
}
