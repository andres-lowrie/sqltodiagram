struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer { input: input }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_input() {
        let got = Lexer::new("input");
        assert_eq!("input", got.input);
    }
}
