use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    kind: TokenType,
}

impl Token {
    pub fn new(kind: TokenType) -> Token {
        Token { kind: kind }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    EOF,
    Unknown,
    Identifer(String),
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
    // Keywords
    KwSelect,
    KwFrom,
}

pub fn keywords() -> HashMap<String, Token> {
    let mut tokens: HashMap<String, Token> = HashMap::new();

    // Non-Alphanum
    tokens.insert(String::from("+"), Token::new(TokenType::Plus));
    tokens.insert(String::from("*"), Token::new(TokenType::Asterisk));
    tokens.insert(String::from("/"), Token::new(TokenType::Slash));
    tokens.insert(String::from("<"), Token::new(TokenType::LeftAngle));
    tokens.insert(String::from(">"), Token::new(TokenType::RightAngle));
    tokens.insert(String::from("="), Token::new(TokenType::Equals));
    tokens.insert(String::from("~"), Token::new(TokenType::Tilde));
    tokens.insert(String::from("!"), Token::new(TokenType::Bang));
    tokens.insert(String::from("@"), Token::new(TokenType::At));
    tokens.insert(String::from("#"), Token::new(TokenType::Hash));
    tokens.insert(String::from("%"), Token::new(TokenType::Percent));
    tokens.insert(String::from("^"), Token::new(TokenType::Caret));
    tokens.insert(String::from("&"), Token::new(TokenType::Ampersand));
    tokens.insert(String::from("|"), Token::new(TokenType::Pipe));
    tokens.insert(String::from("`"), Token::new(TokenType::Backtick));
    tokens.insert(String::from("?"), Token::new(TokenType::Question));
    tokens.insert(String::from("-"), Token::new(TokenType::Hyphen));
    tokens.insert(String::from("$"), Token::new(TokenType::Dollar));
    tokens.insert(String::from("("), Token::new(TokenType::LeftParen));
    tokens.insert(String::from(")"), Token::new(TokenType::RightParen));
    tokens.insert(String::from("["), Token::new(TokenType::LeftBracket));
    tokens.insert(String::from("]"), Token::new(TokenType::RightBracket));
    tokens.insert(String::from(","), Token::new(TokenType::Comma));
    tokens.insert(String::from(":"), Token::new(TokenType::Colon));
    tokens.insert(String::from("."), Token::new(TokenType::Period));
    tokens.insert(String::from("'"), Token::new(TokenType::SingleQuote));
    tokens.insert(String::from("\""), Token::new(TokenType::DoubleQuote));
    // Keywords
    tokens.insert(String::from("select"), Token::new(TokenType::KwSelect));
    tokens.insert(String::from("from"), Token::new(TokenType::KwFrom));
    tokens
}

pub fn terminators() -> HashMap<String, Token> {
    let mut tokens: HashMap<String, Token> = HashMap::new();

    tokens.insert(String::from(" "), Token::new(TokenType::Space));
    tokens.insert(String::from("\t"), Token::new(TokenType::Tab));
    tokens.insert(String::from(";"), Token::new(TokenType::Semicolon));

    tokens
}
