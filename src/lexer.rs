use crate::token;

fn get_tokens(input: &str) -> Vec<token::Token> {
    let avail_tokens = token::tokens();

    let mut found_tokens = Vec::new();

    let mut chars = input.chars();

    loop {
        let ch = chars.next();

        match ch {
            Some(t) => {
                let got = avail_tokens.get(&t);
                match got {
                    Some(g) => {
                        // @TODO This is copying the token, can we get by with just referencing it?
                        found_tokens.push(*g)
                    }
                    None => found_tokens.push(token::Token::new(token::TokenType::Unknown)),
                };
            }
            None => break,
        };
    }

    found_tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_tokenize_operator_tokens() {
        let want = vec![
            token::Token::new(token::TokenType::Plus),
            token::Token::new(token::TokenType::Asterisk),
            token::Token::new(token::TokenType::Slash),
            token::Token::new(token::TokenType::LeftAngle),
            token::Token::new(token::TokenType::RightAngle),
            token::Token::new(token::TokenType::Equals),
            token::Token::new(token::TokenType::Tilde),
            token::Token::new(token::TokenType::Bang),
            token::Token::new(token::TokenType::At),
            token::Token::new(token::TokenType::Hash),
            token::Token::new(token::TokenType::Percent),
            token::Token::new(token::TokenType::Caret),
            token::Token::new(token::TokenType::Ampersand),
            token::Token::new(token::TokenType::Pipe),
            token::Token::new(token::TokenType::Backtick),
            token::Token::new(token::TokenType::Question),
            token::Token::new(token::TokenType::Hyphen),
            token::Token::new(token::TokenType::Dollar),
            token::Token::new(token::TokenType::LeftParen),
            token::Token::new(token::TokenType::RightParen),
            token::Token::new(token::TokenType::LeftBracket),
            token::Token::new(token::TokenType::RightBracket),
            token::Token::new(token::TokenType::Comma),
            token::Token::new(token::TokenType::Semicolon),
            token::Token::new(token::TokenType::Colon),
            token::Token::new(token::TokenType::Period),
        ];
        let got = get_tokens("+*/<>=~!@#%^&|`?-$()[],;:.");
        assert_eq!(want, got);
    }

    #[test]
    fn it_can_tokenize_quotes() {
        let want = vec![
            token::Token::new(token::TokenType::SingleQuote),
            token::Token::new(token::TokenType::DoubleQuote),
        ];
        let got = get_tokens("'\"");
        assert_eq!(want, got);
    }

    #[test]
    fn it_can_tokenize_whitespace() {
        let want = vec![
            token::Token::new(token::TokenType::Space),
            token::Token::new(token::TokenType::Tab),
            token::Token::new(token::TokenType::Newline),
        ];
        let got = get_tokens(" \t\n");
        assert_eq!(want, got);
    }

}
