use crate::token::*;

fn str_or_id(mut word: String) -> TokenType {
    if "'\"".contains(word.chars().next().unwrap()) {
        word.retain(|c| !"'\"".contains(c));
        return TokenType::Str(word);
    }
    TokenType::Identifer(word)
}

fn get_tokens(input: &str) -> Vec<Token> {
    //let operators = _oken::operators();
    let terminators = terminators();
    let keywords = keywords();

    // Input
    let mut chars = input.chars();

    // Output
    let mut found_tokens = Vec::new();

    loop {
        let mut ch = chars.next();

        match ch {
            Some(t) => {
                // It should parse out keywords by consuming chars and identifying when each
                // keyword or identifer ends
                let mut word = String::from("");
                word.push(t);

                loop {
                    ch = chars.next();
                    if let Some(c) = ch {
                        if terminators.iter().any(|(k, _)| *k == c.to_string()) {
                            match keywords.get(&word) {
                                Some(w) => found_tokens.push(w.clone()),
                                //None => found_tokens.push(Token::new(TokenType::Identifer(word))),
                                None => {
                                    found_tokens.push(Token::new(str_or_id(word)));
                                }
                            }
                            break;
                        } else {
                            word.push(c);
                        }
                    } else {
                        // This should catche the case where the token is that last thing in
                        // the input ie: no new line extra whitespace
                        match keywords.get(&word) {
                            Some(w) => found_tokens.push(w.clone()),
                            //None => found_tokens.push(Token::new(TokenType::Identifer(word))),
                            None => {
                                found_tokens.push(Token::new(str_or_id(word)));
                            }
                        }
                        break;
                    }
                }
            }
            None => {
                found_tokens.push(Token::new(TokenType::EOF));
                break;
            }
        };
    }

    found_tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_tokenize_hello_world() {
        let want = vec![
            Token::new(TokenType::KwSelect),
            Token::new(TokenType::Asterisk),
            Token::new(TokenType::KwFrom),
            Token::new(TokenType::Identifer(String::from("tbl"))),
            Token::new(TokenType::EOF),
        ];

        let got = get_tokens("select * from tbl;");
        assert_eq!(want, got);
    }

    #[test]
    fn it_can_tokenize_strings() {
        let want = vec![
            Token::new(TokenType::KwSelect),
            Token::new(TokenType::Asterisk),
            Token::new(TokenType::KwFrom),
            Token::new(TokenType::Identifer(String::from("tbl"))),
            Token::new(TokenType::KwWhere),
            Token::new(TokenType::Identifer(String::from("col"))),
            Token::new(TokenType::Equals),
            Token::new(TokenType::Str(String::from("foobar"))),
            Token::new(TokenType::EOF),
        ];

        let got = get_tokens("select * from tbl where col = 'foobar'");
        assert_eq!(want, got);
    }
}
