#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // --- Control Flow Keywords ---
    Return,
    If,
    Else,
    While,
    Do,
    For,
    Break,
    Continue,
    Goto,

    // --- Modifier & Visibility Keywords ---
    Public,
    Protected,
    Private,
    Async,

    // --- Built-in Data Types ---
    Void,
    Int,
    Float,
    Char,
    Bool,

    // --- Boolean Literals ---
    True,
    False,

    // --- Operators ---
    Assign, // =
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /

    // --- Logical Operators ---
    And, // &&
    Or,  // ||
    Not, // !

    // --- Relational Operators ---
    Eq,    // ==
    NotEq, // !=
    Lt,    // <
    Gt,    // >
    LtEq,  // <=
    GtEq,  // >=

    // --- Bitwise Operators ---
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    BitNot, // ~
    Shl,    // << (Shift left)
    Shr,    // >> (Shift right)

    // --- Punctuation & Delimitators ---
    Semi,         // ;
    Comma,        // ,
    Colon,        // :
    OpenParen,    // (
    CloseParen,   // )
    OpenCurly,    // {
    CloseCurly,   // }
    OpenBracket,  // [
    CloseBracket, // ]

    // --- Dynamic Literals and Identifiers ---
    IntLit,   // e.g., 42
    FloatLit, // e.g., 3.14
    CharLit,  // e.g., 'a'
    Ident,    // Variable or function names, e.g., "my_variable"
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(value: Option<String>, token_type: TokenType) -> Self {        
        Token { token_type, value, line: 0, column: 0 }
    }
}
