pub enum OperatorToken {
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    LessThan, // <
    // LessThanOrEqual, // <
    GreaterThan, // >
                 // GreaterThanOrEqual, // >=
}

pub enum DelimiterToken {
    Comma,        // ,
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
}

pub enum KeywordToken {
    Function, // fn
    Let,      // let
}

impl KeywordToken {
    pub fn lookup(str: &str) -> Option<Self> {
        Some(match str {
            "fn" => KeywordToken::Function,
            "let" => KeywordToken::Let,
            _ => return None,
        })
    }
}

pub enum Token {
    Eof,

    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Keyword(KeywordToken),

    // values
    Id(String),
    Number(String),
}
