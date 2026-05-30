use std::{iter::Peekable, str::Chars};

use crate::compiler::token::{Token, TokenType};

fn read_identifier_or_keyword(chars: &mut Peekable<Chars>) -> Token {
    let mut buf = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            buf.push(c);
            chars.next();
        } else {
            break;
        }
    }

    let token_type: TokenType = match buf.as_str() {
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

    Token::new(Some(buf), token_type)
}

fn read_number_literal(chars: &mut Peekable<Chars>) -> Token {
    let mut buf = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            buf.push(c);
            chars.next();
        } else {
            break;
        }
    }

    Token::new(Some(buf), TokenType::IntLit)
}

pub fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = str.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_alphabetic() || c == '_' {
            let token = read_identifier_or_keyword(&mut chars);
            tokens.push(token);
        } else if c.is_numeric() {
            let token = read_number_literal(&mut chars);
            tokens.push(token);
        } else {
            chars.next();
        }
    }

    tokens
}
