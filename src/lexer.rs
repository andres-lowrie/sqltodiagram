use crate::token::*;

fn str_or_id(mut word: String) -> TokenType {
    if "'\"".contains(word.chars().next().unwrap()) {
        word.retain(|c| !"'\"".contains(c));
        return TokenType::Str(word);
    }
    TokenType::Identifer(word)
}

fn in_keywords(word: String) -> bool {
    let keywords = keywords();
    if keywords.keys().any(|k| k == &word.to_string()) {
        return true;
    }
    false
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
        let mut cur = chars.next();
        println!("Outter After {:?}", cur);

        match cur {
            Some(' ') => { /* noop */ }
            Some('\t') => { /* noop */ }
            Some(';') => { /* noop */ }
            Some(wrd) if in_keywords(wrd.to_string()) => {
                println!("In t match {:?}, {:?}", wrd, cur);
                if let Some(kw) = keywords.get(&wrd.to_string()) {
                    found_tokens.push(kw.clone());
                }
                println!("After t match cur: {:?}", cur);
            }
            Some(_) => {
                println!("In underscore match {:?}", cur);
                // It should parse out keywords by consuming chars and identifying when each
                // keyword or identifer ends
                let mut word = String::from("");
                //println!("Top Pushing {:?}", t);

                loop {
                    if let Some(c) = cur {
                        // Is this the end of the word?
                        if terminators.iter().any(|(k, _)| *k == c.to_string()) || c == ')' {
                            println!("Found Terminator {:?}", c);
                            break;
                        }

                        println!("Pushin {:?}", c);
                        word.push(c);
                        cur = chars.next();
                    } else {
                        println!("Before loop break {:?}", word);
                        break;
                    }
                }

                println!("Before kwrd match {:?} {:?}", word, cur);
                match keywords.get(&word) {
                    Some(w) => {
                        println!("Found word {:?}\n", w);
                        found_tokens.push(w.clone());
                    }
                    None => {
                        println!("Found str/id {:?}\n", word);
                        found_tokens.push(Token::new(str_or_id(word.clone())));
                        // -- YEAH.... this //
                        if let Some(c) = cur {
                            if !terminators.iter().any(|(k, _)| *k == c.to_string()) {
                                println!("Found non terminator at `cur` after word {:?}\n", cur);
                                if let Some(kw) = keywords.get(&c.to_string()) {
                                    found_tokens.push(kw.clone());
                                }
                            }
                        }
                        // --
                    }
                }
            }
            None => {
                println!("In Top level none {:?}", cur);
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

    #[test]
    fn it_can_tokenize_parens() {
        let want = vec![
            Token::new(TokenType::KwSelect),
            Token::new(TokenType::Asterisk),
            Token::new(TokenType::KwFrom),
            Token::new(TokenType::LeftParen),
            Token::new(TokenType::KwSelect),
            Token::new(TokenType::Identifer(String::from("a"))),
            Token::new(TokenType::KwFrom),
            Token::new(TokenType::Identifer(String::from("b"))),
            Token::new(TokenType::KwWhere),
            Token::new(TokenType::Identifer(String::from("c"))),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::EOF),
        ];

        let cases = vec![
            "select * from (select a from b where c)",
            "select * from (select a from b where c);",
            "select * from ( select a from b where c )",
            "select * from ( select a from b where c );",
        ];

        for c in cases {
            let got = get_tokens(c);
            assert_eq!(want, got);
        }
    }
}
