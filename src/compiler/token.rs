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

    // --- Dynamic Literals and Identifiers ---
    IntLit,      // e.g., 42
    FloatLit,    // e.g., 3.14
    CharLit,     // e.g., 'a'
    Ident,       // Variable or function names, e.g., "my_variable"

    // --- Operators ---
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    
    // --- Logical Operators ---
    And,         // &&
    Or,          // ||
    Not,         // !
    
    // --- Relational Operators ---
    Eq,          // ==
    NotEq,       // !=
    Lt,          // <
    Gt,          // >

    // --- Bitwise Operators ---
    BitAnd,      // &
    BitOr,       // |
    BitXor,      // ^
    BitNot,      // ~
    Shl,         // << (Shift left)
    Shr,         // >> (Shift right)

    // --- Punctuation & Delimitators ---
    Semi,        // ;
    Comma,       // ,
    Colon,       // :
    OpenParen,   // (
    CloseParen,  // )
    OpenCurly,   // {
    CloseCurly,  // }
    OpenBracket, // [
    CloseBracket,// ]
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub fn tokenize(str: &str) -> Vec<Token> {
    let tokens = Vec::new();

    let mut chars = str.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() {

            let mut buf = String::new();

            buf.push(c);
            chars.next();

            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() {
                    buf.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }

            println!("Word captured: {}", buf);
        } else {
            chars.next();
        }
    }

    tokens
}
