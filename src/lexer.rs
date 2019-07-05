use crate::token::*;

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
                // @TODO change the approach here
                // It should parse out keywords by consuming chars and identifying when each
                // keyword or identifer ends
                //
                // Something like
                // Check for single character symbols first if none are found then search for
                // keywords
                let mut word = String::from("");
                if terminators.iter().any(|(k, _)| *k == t.to_string()) {
                    println!("End of word top level");
                } else {
                    word.push(t);
                    loop {
                        ch = chars.next();
                        if let Some(c) = ch {
                            //println!("Some c from ch");
                            //print!(" {:?}", c);
                            if terminators.iter().any(|(k, _)| *k == c.to_string()) {
                                println!("End of word in loop");
                                match keywords.get(&word) {
                                    Some(w) => found_tokens.push(w.clone()),
                                    None => {
                                        found_tokens.push(Token::new(TokenType::Identifer(word)))
                                    }
                                }
                                break;
                            } else {
                                word.push(c);
                            }
                        } else {
                            // This should catche the case where the token is that last thing in
                            // the input ie: no new line or extra whitespace
                            println!("End of nested loop");
                            println!("{:?}", word);
                            match keywords.get(&word) {
                                Some(w) => found_tokens.push(w.clone()),
                                None => found_tokens.push(Token::new(TokenType::Identifer(word))),
                            }
                            //print!("word {:?}", word);
                            break;
                        }
                    }
                }
                //println!("word {:?}", word);
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
            Token::new(TokenType::Identifer(String::from("table"))),
            Token::new(TokenType::EOF),
        ];

        let got = get_tokens("select * from table;");
        assert_eq!(want, got);
    }
}
