use std::collections::HashMap;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token {
    kind: TokenType,
}

impl Token {
    pub fn new(kind: TokenType) -> Token {
        Token { kind: kind }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Unknown,
    Identifer,
    Plus,
    Minus,
    //BANG,
    //NUMBER,
    //STRING,
    //AND,
    //OR,
    //EQUALS,
    //PLUS,
    //MINUS,
    //PERIOD,
    //COMMA,
    //SLASH,
    //COLON,
    //SEMICOLON,
    //QUESTION,
    //UNDERSCORE,
    //PIPE,
    //AMPERSAND,
    //PERCENT,
    //ASTERISK,

    //LEFTPAREN,
    //RIGHTPAREN,
    //LEFTANGLE,
    //RIGHTANGLE,
    //LEFTBRACKET,
    //RIGHTBRACKET,
}

pub fn tokens() -> HashMap<char, Token> {
    let mut tokens: HashMap<char, Token> = HashMap::new();

    tokens.insert('+', Token::new(TokenType::Plus));
    tokens.insert('-', Token::new(TokenType::Minus));
    return tokens;
}
