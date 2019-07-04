#[derive(PartialEq, Debug)]
pub struct Token {
    typ: TokenType,
}

impl Token {
    pub fn new(typ: TokenType) -> Token {
        Token { typ: typ }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    //UNKNOWN,
    //EOF,
    Identifer,
    //SELECT,
    //FROM,
    //AS,
    //WHERE,
    //GROUP,
    //BY,
    //HAVING,
    //JOIN,
    //UNION,
    //CROSS,
    //LEFT,
    //RIGHT,
    //INNER,
    //OUTTER,
    //FULL,
    //ON,

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
