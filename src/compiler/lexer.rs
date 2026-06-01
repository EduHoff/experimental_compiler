use std::{iter::Peekable, str::Chars};

use crate::compiler::token::{
    Token,
    TokenType::{self},
};

fn advance(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) {
    let c = chars.peek();

    if c == Some(&'\n') {
        *line += 1;
        *column = 1;
        chars.next();
    } else {
        *column += 1;
        chars.next();
    }
}

fn skip_whitespace(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            advance(chars, line, column);
        } else {
            break;
        }
    }
}

fn read_identifier_or_keyword(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
    column: &mut usize,
) -> Token {
    let mut buf = String::new();

    let current_line = *line;
    let current_column = *column;

    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            buf.push(c);
            advance(chars, line, column);
        } else {
            break;
        }
    }

    let token_type: TokenType = match buf.as_str() {
        "import" => TokenType::Import,
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
        "double" => TokenType::Double,
        "char" => TokenType::Char,
        "bool" => TokenType::Bool,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Ident,
    };

    Token::new(Some(buf), token_type, current_line, current_column)
}

fn read_number_literal(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let mut buf = String::new();

    let current_line = *line;
    let current_column = *column;
    let mut has_dot = 0;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            if c == '.' {
                has_dot += 1;
            }

            if has_dot > 1 {
                break;
            }

            buf.push(c);
            advance(chars, line, column);
        } else {
            break;
        }
    }

    let token_type = if buf.contains('.') {
        TokenType::DoubleLit
    } else {
        TokenType::IntLit
    };

    Token::new(Some(buf), token_type, current_line, current_column)
}

fn read_operator_or_punctuation(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
    column: &mut usize,
) -> Token {
    let mut buf = String::new();

    let current_line = *line;
    let current_column = *column;

    if let Some(&c) = chars.peek() {
        buf.push(c);

        let token_type = match c {
            ';' => Some(TokenType::Semi),
            ',' => Some(TokenType::Comma),
            ':' => Some(TokenType::Colon),
            '(' => Some(TokenType::OpenParen),
            ')' => Some(TokenType::CloseParen),
            '{' => Some(TokenType::OpenCurly),
            '}' => Some(TokenType::CloseCurly),
            '[' => Some(TokenType::OpenBracket),
            ']' => Some(TokenType::CloseBracket),
            '^' => Some(TokenType::BitXor),
            '~' => Some(TokenType::BitNot),
            _ => None,
        };

        if let Some(t_type) = token_type {
            advance(chars, line, column);
            return Token::new(Some(buf), t_type, current_line, current_column);
        }

        match buf.as_str() {
            "=" => {}
            "+" => {}
            "-" => {}
            "*" => {}
            "/" => {}
            "%" => {}
            "&" => {}
            "|" => {}
            "!" => {}
            "<" => {}
            ">" => {}
            _ => {}
        }
    }

    advance(chars, line, column);
    Token::new(Some(buf), TokenType::Invalid, current_line, current_column)
}

pub fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut line: usize = 1;
    let mut column: usize = 1;

    let mut chars = str.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            skip_whitespace(&mut chars, &mut line, &mut column);
        } else if c.is_alphabetic() || c == '_' {
            let token = read_identifier_or_keyword(&mut chars, &mut line, &mut column);
            tokens.push(token);
        } else if c.is_numeric() {
            let token = read_number_literal(&mut chars, &mut line, &mut column);
            tokens.push(token);
        } else {
            let token = read_operator_or_punctuation(&mut chars, &mut line, &mut column);
            tokens.push(token);
        }
    }

    tokens
}
