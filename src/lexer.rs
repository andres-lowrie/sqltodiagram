use crate::token;

fn get_tokens(input: &str) -> Vec<token::Token> {
    let mut tokens = Vec::new();

    let mut chars = input.chars();
    let ch = chars.next();

    let x = match ch {
        Some(_) => token::Token::new(token::TokenType::Identifer),
        None => panic!("oops"),
    };

    tokens.push(x);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_return_tokens() {
        let want = vec![token::Token::new(token::TokenType::Identifer)];
        let got = get_tokens("a");
        assert_eq!(want, got);
    }

}
