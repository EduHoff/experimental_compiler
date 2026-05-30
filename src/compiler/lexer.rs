use crate::compiler::token::{Token, TokenType};


fn match_token_type(value: &Option<String>) -> TokenType {
    match value.as_deref() {
        // --- Control Flow Keywords ---
        Some("return") => TokenType::Return,
        Some("if") => TokenType::If,
        Some("else") => TokenType::Else,
        Some("while") => TokenType::While,
        Some("do") => TokenType::Do,
        Some("for") => TokenType::For,
        Some("break") => TokenType::Break,
        Some("continue") => TokenType::Continue,
        Some("goto") => TokenType::Goto,

        // --- Modifier & Visibility Keywords ---
        Some("public") => TokenType::Public,
        Some("protected") => TokenType::Protected,
        Some("private") => TokenType::Private,
        Some("async") => TokenType::Async,

        // --- Built-in Data Types ---
        Some("void") => TokenType::Void,
        Some("int") => TokenType::Int,
        Some("float") => TokenType::Float,
        Some("char") => TokenType::Char,
        Some("bool") => TokenType::Bool,

        // --- Boolean Literals ---
        Some("true") => TokenType::True,
        Some("false") => TokenType::False,

        // --- Operators ---
        Some("=") => TokenType::Assign,
        Some("+") => TokenType::Plus,
        Some("-") => TokenType::Minus,
        Some("*") => TokenType::Star,
        Some("/") => TokenType::Slash,

        // --- Logical Operators ---
        Some("&&") => TokenType::And,
        Some("||") => TokenType::Or,
        Some("!") => TokenType::Not,

        // --- Relational Operators ---
        Some("==") => TokenType::Eq,
        Some("!=") => TokenType::NotEq,
        Some("<") => TokenType::Lt,
        Some(">") => TokenType::Gt,
        Some("<=") => TokenType::LtEq,
        Some(">=") => TokenType::GtEq,

        // --- Bitwise Operators ---
        Some("&") => TokenType::BitAnd,
        Some("|") => TokenType::BitOr,
        Some("^") => TokenType::BitXor,
        Some("~") => TokenType::BitNot,
        Some("<<") => TokenType::Shl,
        Some(">>") => TokenType::Shr,

        // --- Punctuation & Delimitators ---
        Some(";") => TokenType::Semi,
        Some(",") => TokenType::Comma,
        Some(":") => TokenType::Colon,
        Some("(") => TokenType::OpenParen,
        Some(")") => TokenType::CloseParen,
        Some("{") => TokenType::OpenCurly,
        Some("}") => TokenType::CloseCurly,
        Some("[") => TokenType::OpenBracket,
        Some("]") => TokenType::CloseBracket,

        // --- Dynamic Literals and Identifiers ---
        // ainda preciso pensar como fazer isso...
        _ => TokenType::Ident, 
    }
}


pub fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = str.chars().peekable();

    // impl Iterator<Item = char>

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

            let token_type = match buf.as_str() {
                "return" => TokenType::Return,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "while" => TokenType::While,
                "do" => TokenType::Do,
                "for" => TokenType::For,
                "break" => TokenType::Break,
                "continue" => TokenType::Continue,
                "goto" => TokenType::Goto,
                "public" => TokenType::Public,
                "protected" => TokenType::Protected,
                "private" => TokenType::Private,
                "async" => TokenType::Async,
                "void" => TokenType::Void,
                "int" => TokenType::Int,
                "float" => TokenType::Float,
                "char" => TokenType::Char,
                "bool" => TokenType::Bool,
                "true" => TokenType::True,
                "false" => TokenType::False,
                _ => TokenType::Ident,
            };

            let value = if token_type == TokenType::Ident {
                Some(buf)
            } else {
                None
            };

            //tokens.push(Token { token_type, value });
        } else {
            chars.next();
        }
    }

    tokens
}
