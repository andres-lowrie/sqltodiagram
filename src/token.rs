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
    // Non-Alphanum
    Plus,
    Asterisk,
    Slash,
    LeftAngle,
    RightAngle,
    Equals,
    Tilde,
    Bang,
    At,
    Hash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Backtick,
    Question,
    Hyphen,
    Dollar,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Period,
    SingleQuote,
    DoubleQuote,
    Space,
    Tab,
    Newline,
    // Alphnum
    Alpha,
    Number,
}

pub fn tokens() -> HashMap<char, Token> {
    let mut tokens: HashMap<char, Token> = HashMap::new();

    tokens.insert('+', Token::new(TokenType::Plus));
    tokens.insert('*', Token::new(TokenType::Asterisk));
    tokens.insert('/', Token::new(TokenType::Slash));
    tokens.insert('<', Token::new(TokenType::LeftAngle));
    tokens.insert('>', Token::new(TokenType::RightAngle));
    tokens.insert('=', Token::new(TokenType::Equals));
    tokens.insert('~', Token::new(TokenType::Tilde));
    tokens.insert('!', Token::new(TokenType::Bang));
    tokens.insert('@', Token::new(TokenType::At));
    tokens.insert('#', Token::new(TokenType::Hash));
    tokens.insert('%', Token::new(TokenType::Percent));
    tokens.insert('^', Token::new(TokenType::Caret));
    tokens.insert('&', Token::new(TokenType::Ampersand));
    tokens.insert('|', Token::new(TokenType::Pipe));
    tokens.insert('`', Token::new(TokenType::Backtick));
    tokens.insert('?', Token::new(TokenType::Question));
    tokens.insert('-', Token::new(TokenType::Hyphen));
    tokens.insert('$', Token::new(TokenType::Dollar));
    tokens.insert('(', Token::new(TokenType::LeftParen));
    tokens.insert(')', Token::new(TokenType::RightParen));
    tokens.insert('[', Token::new(TokenType::LeftBracket));
    tokens.insert(']', Token::new(TokenType::RightBracket));
    tokens.insert(',', Token::new(TokenType::Comma));
    tokens.insert(';', Token::new(TokenType::Semicolon));
    tokens.insert(':', Token::new(TokenType::Colon));
    tokens.insert('.', Token::new(TokenType::Period));
    tokens.insert('\'', Token::new(TokenType::SingleQuote));
    tokens.insert('"', Token::new(TokenType::DoubleQuote));
    tokens.insert(' ', Token::new(TokenType::Space));
    tokens.insert('\t', Token::new(TokenType::Tab));
    tokens.insert('\n', Token::new(TokenType::Newline));
    tokens
}
