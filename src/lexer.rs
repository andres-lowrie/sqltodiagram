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
    fn it_can_return_correct_tokens() {
        let want = vec![
            token::Token::new(token::TokenType::Plus),
            token::Token::new(token::TokenType::Minus),
        ];
        let got = get_tokens("+-");
        assert_eq!(want, got);
    }

}
