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
    // Operators
    And,
    Or,
    Not,
    // Keywords
    KwAny,
    KwAll,
    KwAnalyze,
    KwArray,
    KwAs,
    KwAsc,
    KwAsymmetric,
    KwAuthorization,
    KwBetween,
    KwBinary,
    KwBoth,
    KwCase,
    KwCast,
    KwCheck,
    KwCollate,
    KwCollation,
    KwColumn,
    KwConcurrently,
    KwConstraint,
    KwCreate,
    KwCross,
    KwCurrentCatalog,
    KwCurrentDate,
    KwCurrentRole,
    KwCurrentSchema,
    KwCurrentTime,
    KwCurrentUser,
    KwDefault,
    KwDeferrable,
    KwDesc,
    KwDistinct,
    KwDo,
    KwElse,
    KwEnd,
    KwExcept,
    KwFalse,
    KwFetch,
    KwFor,
    KwForeign,
    KwFreeze,
    KwFrom,
    KwFull,
    KwGrant,
    KwGroup,
    KwHaving,
    KwIlike,
    KwIn,
    KwInitially,
    KwInner,
    KwIntersect,
    KwInto,
    KwIs,
    KwIsnull,
    KwJoin,
    KwLateral,
    KwLeading,
    KwLeft,
    KwLike,
    KwLimit,
    KwLocaltime,
    KwNatural,
    KwNotnull,
    KwNull,
    KwOffset,
    KwOn,
    KwOnly,
    KwOrder,
    KwOuter,
    KwOverlaps,
    KwPlacing,
    KwPrimary,
    KwReferences,
    KwReturning,
    KwRight,
    KwSelect,
    KwSessionUser,
    KwSimilar,
    KwSome,
    KwSymmetric,
    KwTable,
    KwTablesample,
    KwThen,
    KwTo,
    KwTrailing,
    KwTrue,
    KwUnion,
    KwUnique,
    KwUser,
    KwUsing,
    KwVariadic,
    KwVerbose,
    KwWhen,
    KwWhere,
    KwWindow,
    KwWith,
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
    // https://www.postgresql.org/docs/9.0/sql-keywords-appendix.html
    tokens.insert(String::from("all"), Token::new(TokenType::KwAll));
    tokens.insert(String::from("analyse"), Token::new(TokenType::KwAnalyze));
    tokens.insert(String::from("analyze"), Token::new(TokenType::KwAnalyze));
    tokens.insert(String::from("and"), Token::new(TokenType::And));
    tokens.insert(String::from("any"), Token::new(TokenType::KwAny));
    tokens.insert(String::from("array"), Token::new(TokenType::KwArray));
    tokens.insert(String::from("as"), Token::new(TokenType::KwAs));
    tokens.insert(String::from("asc"), Token::new(TokenType::KwAsc));
    tokens.insert(
        String::from("asymmetric"),
        Token::new(TokenType::KwAsymmetric),
    );
    tokens.insert(
        String::from("authorization"),
        Token::new(TokenType::KwAuthorization),
    );
    tokens.insert(String::from("between"), Token::new(TokenType::KwBetween));
    tokens.insert(String::from("binary"), Token::new(TokenType::KwBinary));
    tokens.insert(String::from("both"), Token::new(TokenType::KwBoth));
    tokens.insert(String::from("case"), Token::new(TokenType::KwCase));
    tokens.insert(String::from("cast"), Token::new(TokenType::KwCast));
    tokens.insert(String::from("check"), Token::new(TokenType::KwCheck));
    tokens.insert(String::from("collate"), Token::new(TokenType::KwCollate));
    tokens.insert(
        String::from("collation"),
        Token::new(TokenType::KwCollation),
    );
    tokens.insert(String::from("column"), Token::new(TokenType::KwColumn));
    tokens.insert(
        String::from("concurrently"),
        Token::new(TokenType::KwConcurrently),
    );
    tokens.insert(
        String::from("constraint"),
        Token::new(TokenType::KwConstraint),
    );
    tokens.insert(String::from("create"), Token::new(TokenType::KwCreate));
    tokens.insert(String::from("cross"), Token::new(TokenType::KwCross));
    tokens.insert(
        String::from("current_catalog"),
        Token::new(TokenType::KwCurrentCatalog),
    );
    tokens.insert(
        String::from("current_date"),
        Token::new(TokenType::KwCurrentDate),
    );
    tokens.insert(
        String::from("current_role"),
        Token::new(TokenType::KwCurrentRole),
    );
    tokens.insert(
        String::from("current_schema"),
        Token::new(TokenType::KwCurrentSchema),
    );
    tokens.insert(
        String::from("current_time"),
        Token::new(TokenType::KwCurrentTime),
    );
    tokens.insert(
        String::from("current_user"),
        Token::new(TokenType::KwCurrentUser),
    );
    tokens.insert(String::from("default"), Token::new(TokenType::KwDefault));
    tokens.insert(
        String::from("deferrable"),
        Token::new(TokenType::KwDeferrable),
    );
    tokens.insert(String::from("desc"), Token::new(TokenType::KwDesc));
    tokens.insert(String::from("distinct"), Token::new(TokenType::KwDistinct));
    tokens.insert(String::from("do"), Token::new(TokenType::KwDo));
    tokens.insert(String::from("else"), Token::new(TokenType::KwElse));
    tokens.insert(String::from("end"), Token::new(TokenType::KwEnd));
    tokens.insert(String::from("except"), Token::new(TokenType::KwExcept));
    tokens.insert(String::from("false"), Token::new(TokenType::KwFalse));
    tokens.insert(String::from("fetch"), Token::new(TokenType::KwFetch));
    tokens.insert(String::from("for"), Token::new(TokenType::KwFor));
    tokens.insert(String::from("foreign"), Token::new(TokenType::KwForeign));
    tokens.insert(String::from("freeze"), Token::new(TokenType::KwFreeze));
    tokens.insert(String::from("from"), Token::new(TokenType::KwFrom));
    tokens.insert(String::from("full"), Token::new(TokenType::KwFull));
    tokens.insert(String::from("grant"), Token::new(TokenType::KwGrant));
    tokens.insert(String::from("group"), Token::new(TokenType::KwGroup));
    tokens.insert(String::from("having"), Token::new(TokenType::KwHaving));
    tokens.insert(String::from("ilike"), Token::new(TokenType::KwIlike));
    tokens.insert(String::from("in"), Token::new(TokenType::KwIn));
    tokens.insert(
        String::from("initially"),
        Token::new(TokenType::KwInitially),
    );
    tokens.insert(String::from("inner"), Token::new(TokenType::KwInner));
    tokens.insert(
        String::from("intersect"),
        Token::new(TokenType::KwIntersect),
    );
    tokens.insert(String::from("into"), Token::new(TokenType::KwInto));
    tokens.insert(String::from("is"), Token::new(TokenType::KwIs));
    tokens.insert(String::from("isnull"), Token::new(TokenType::KwIsnull));
    tokens.insert(String::from("join"), Token::new(TokenType::KwJoin));
    tokens.insert(String::from("lateral"), Token::new(TokenType::KwLateral));
    tokens.insert(String::from("leading"), Token::new(TokenType::KwLeading));
    tokens.insert(String::from("left"), Token::new(TokenType::KwLeft));
    tokens.insert(String::from("like"), Token::new(TokenType::KwLike));
    tokens.insert(String::from("limit"), Token::new(TokenType::KwLimit));
    tokens.insert(
        String::from("localtime"),
        Token::new(TokenType::KwLocaltime),
    );
    tokens.insert(String::from("natural"), Token::new(TokenType::KwNatural));
    tokens.insert(String::from("not"), Token::new(TokenType::Not));
    tokens.insert(String::from("notnull"), Token::new(TokenType::KwNotnull));
    tokens.insert(String::from("null"), Token::new(TokenType::KwNull));
    tokens.insert(String::from("offset"), Token::new(TokenType::KwOffset));
    tokens.insert(String::from("on"), Token::new(TokenType::KwOn));
    tokens.insert(String::from("only"), Token::new(TokenType::KwOnly));
    tokens.insert(String::from("or"), Token::new(TokenType::Or));
    tokens.insert(String::from("order"), Token::new(TokenType::KwOrder));
    tokens.insert(String::from("outer"), Token::new(TokenType::KwOuter));
    tokens.insert(String::from("overlaps"), Token::new(TokenType::KwOverlaps));
    tokens.insert(String::from("placing"), Token::new(TokenType::KwPlacing));
    tokens.insert(String::from("primary"), Token::new(TokenType::KwPrimary));
    tokens.insert(
        String::from("references"),
        Token::new(TokenType::KwReferences),
    );
    tokens.insert(
        String::from("returning"),
        Token::new(TokenType::KwReturning),
    );
    tokens.insert(String::from("right"), Token::new(TokenType::KwRight));
    tokens.insert(String::from("select"), Token::new(TokenType::KwSelect));
    tokens.insert(
        String::from("session_user"),
        Token::new(TokenType::KwSessionUser),
    );
    tokens.insert(String::from("similar"), Token::new(TokenType::KwSimilar));
    tokens.insert(String::from("some"), Token::new(TokenType::KwSome));
    tokens.insert(
        String::from("symmetric"),
        Token::new(TokenType::KwSymmetric),
    );
    tokens.insert(String::from("table"), Token::new(TokenType::KwTable));
    tokens.insert(
        String::from("tablesample"),
        Token::new(TokenType::KwTablesample),
    );
    tokens.insert(String::from("then"), Token::new(TokenType::KwThen));
    tokens.insert(String::from("to"), Token::new(TokenType::KwTo));
    tokens.insert(String::from("trailing"), Token::new(TokenType::KwTrailing));
    tokens.insert(String::from("true"), Token::new(TokenType::KwTrue));
    tokens.insert(String::from("union"), Token::new(TokenType::KwUnion));
    tokens.insert(String::from("unique"), Token::new(TokenType::KwUnique));
    tokens.insert(String::from("user"), Token::new(TokenType::KwUser));
    tokens.insert(String::from("using"), Token::new(TokenType::KwUsing));
    tokens.insert(String::from("variadic"), Token::new(TokenType::KwVariadic));
    tokens.insert(String::from("verbose"), Token::new(TokenType::KwVerbose));
    tokens.insert(String::from("when"), Token::new(TokenType::KwWhen));
    tokens.insert(String::from("where"), Token::new(TokenType::KwWhere));
    tokens.insert(String::from("window"), Token::new(TokenType::KwWindow));
    tokens.insert(String::from("with"), Token::new(TokenType::KwWith));
    tokens
}

pub fn terminators() -> HashMap<String, Token> {
    let mut tokens: HashMap<String, Token> = HashMap::new();

    tokens.insert(String::from(" "), Token::new(TokenType::Space));
    tokens.insert(String::from("\t"), Token::new(TokenType::Tab));
    tokens.insert(String::from(";"), Token::new(TokenType::Semicolon));

    tokens
}
