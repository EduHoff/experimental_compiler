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
    let mut has_fractional_part = false;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            if has_dot == 1 {
                has_fractional_part = true;
            }
            buf.push(c);
            advance(chars, line, column);
        } else if c == '.' {
            has_dot += 1;
            if has_dot > 1 {
                buf.push(c);
                advance(chars, line, column);
                break;
            }
            buf.push(c);
            advance(chars, line, column);
        } else {
            break;
        }
    }

    let token_type = if has_dot == 1 {
        if has_fractional_part {
            TokenType::DoubleLit
        } else {
            TokenType::Invalid
        }
    } else if has_dot > 1 {
        TokenType::Invalid
    } else {
        TokenType::IntLit
    };

    Token::new(Some(buf), token_type, current_line, current_column)
}

fn read_string(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let mut buf = String::new();

    let current_line = *line;
    let current_column = *column - 1;

    let mut closed = false;

    while let Some(&c) = chars.peek() {
        if c == '\\' {
            buf.push(c);
            advance(chars, line, column);

            if let Some(&escaped) = chars.peek() {
                buf.push(escaped);
                advance(chars, line, column);
            }
        } else if c == '"' {
            advance(chars, line, column);
            closed = true;
            break;
        } else if c == '\n' {
            break;
        } else {
            buf.push(c);
            advance(chars, line, column);
        }
    }

    if closed {
        Token::new(
            Some(buf.clone()),
            TokenType::StringLit,
            current_line,
            current_column,
        )
    } else {
        Token::new(
            Some(buf.clone()),
            TokenType::Invalid,
            current_line,
            current_column,
        )
    }
}

fn read_char(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let mut buf = String::new();

    let current_line = *line;
    let current_column = *column - 1;

    let mut content_count = 0;
    let mut closed = false;

    while let Some(&c) = chars.peek() {
        if c == '\\' {
            if content_count > 0 {
                break;
            }

            buf.push(c);
            advance(chars, line, column);

            if let Some(&escaped) = chars.peek() {
                buf.push(escaped);
                advance(chars, line, column);
                content_count += 1;
            }
        } else if c == '\'' {
            advance(chars, line, column);
            closed = true;
            break;
        } else if c == '\n' {
            break;
        } else {
            if content_count >= 1 {
                break;
            }

            buf.push(c);
            advance(chars, line, column);
            content_count += 1;
        }
    }

    if closed && content_count == 1 {
        Token::new(Some(buf), TokenType::CharLit, current_line, current_column)
    } else {
        Token::new(Some(buf), TokenType::Invalid, current_line, current_column)
    }
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
        let token_type = match c {
            ';' => Some(TokenType::Semi),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            ':' => Some(TokenType::Colon),
            '(' => Some(TokenType::OpenParen),
            ')' => Some(TokenType::CloseParen),
            '{' => Some(TokenType::OpenCurly),
            '}' => Some(TokenType::CloseCurly),
            '[' => Some(TokenType::OpenBracket),
            ']' => Some(TokenType::CloseBracket),
            '~' => Some(TokenType::BitNot),
            _ => None,
        };

        if let Some(t_type) = token_type {
            buf.push(c);
            advance(chars, line, column);
            return Token::new(Some(buf), t_type, current_line, current_column);
        }

        buf.push(c);
        advance(chars, line, column);

        match buf.as_str() {
            "=" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(Some(buf), TokenType::Eq, current_line, current_column);
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Assign, current_line, current_column);
                }
            },
            "+" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::PlusAssign,
                        current_line,
                        current_column,
                    );
                }
                Some(&'+') => {
                    buf.push('+');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::Increment,
                        current_line,
                        current_column,
                    );
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Plus, current_line, current_column);
                }
            },
            "-" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::MinusAssign,
                        current_line,
                        current_column,
                    );
                }
                Some(&'-') => {
                    buf.push('-');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::Decrement,
                        current_line,
                        current_column,
                    );
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Minus, current_line, current_column);
                }
            },
            "*" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::StarAssign,
                        current_line,
                        current_column,
                    );
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Star, current_line, current_column);
                }
            },
            "/" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::SlashAssign,
                        current_line,
                        current_column,
                    );
                }
                Some(&'/') => {
                    buf.push('/');
                    advance(chars, line, column);

                    while let Some(&next_c) = chars.peek() {
                        if next_c == '\n' {
                            break;
                        }
                        buf.push(next_c);
                        advance(chars, line, column);
                    }
                    return Token::new(
                        Some(buf),
                        TokenType::LineComment,
                        current_line,
                        current_column,
                    );
                }
                Some(&'*') => {
                    buf.push('*');
                    advance(chars, line, column);

                    let mut closed = false;

                    while let Some(&next_c) = chars.peek() {
                        buf.push(next_c);
                        advance(chars, line, column);

                        if next_c == '*'
                            && let Some(&'/') = chars.peek()
                        {
                            buf.push('/');
                            advance(chars, line, column);
                            closed = true;
                            break;
                        }
                    }

                    if closed {
                        return Token::new(
                            Some(buf),
                            TokenType::BlockComment,
                            current_line,
                            current_column,
                        );
                    } else {
                        return Token::new(
                            Some(buf),
                            TokenType::Invalid,
                            current_line,
                            current_column,
                        );
                    }
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Slash, current_line, current_column);
                }
            },
            "%" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::ModAssign,
                        current_line,
                        current_column,
                    );
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Modulo, current_line, current_column);
                }
            },
            "&" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::AndAssign,
                        current_line,
                        current_column,
                    );
                }
                Some(&'&') => {
                    buf.push('&');
                    advance(chars, line, column);
                    return Token::new(Some(buf), TokenType::And, current_line, current_column);
                }
                _ => {
                    return Token::new(Some(buf), TokenType::BitAnd, current_line, current_column);
                }
            },
            "|" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::OrAssign,
                        current_line,
                        current_column,
                    );
                }
                Some(&'|') => {
                    buf.push('|');
                    advance(chars, line, column);
                    return Token::new(Some(buf), TokenType::Or, current_line, current_column);
                }
                _ => {
                    return Token::new(Some(buf), TokenType::BitOr, current_line, current_column);
                }
            },
            "^" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(
                        Some(buf),
                        TokenType::XorAssign,
                        current_line,
                        current_column,
                    );
                }
                _ => {
                    return Token::new(Some(buf), TokenType::BitXor, current_line, current_column);
                }
            },
            "!" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(Some(buf), TokenType::NotEq, current_line, current_column);
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Not, current_line, current_column);
                }
            },
            "<" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(Some(buf), TokenType::LtEq, current_line, current_column);
                }
                Some(&'<') => {
                    buf.push('<');
                    advance(chars, line, column);

                    if let Some(&'=') = chars.peek() {
                        buf.push('=');
                        advance(chars, line, column);
                        return Token::new(
                            Some(buf),
                            TokenType::ShlAssign,
                            current_line,
                            current_column,
                        );
                    }

                    return Token::new(Some(buf), TokenType::Shl, current_line, current_column);
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Lt, current_line, current_column);
                }
            },
            ">" => match chars.peek() {
                Some(&'=') => {
                    buf.push('=');
                    advance(chars, line, column);
                    return Token::new(Some(buf), TokenType::GtEq, current_line, current_column);
                }
                Some(&'>') => {
                    buf.push('>');
                    advance(chars, line, column);

                    if let Some(&'=') = chars.peek() {
                        buf.push('=');
                        advance(chars, line, column);
                        return Token::new(
                            Some(buf),
                            TokenType::ShrAssign,
                            current_line,
                            current_column,
                        );
                    }

                    return Token::new(Some(buf), TokenType::Shr, current_line, current_column);
                }
                _ => {
                    return Token::new(Some(buf), TokenType::Gt, current_line, current_column);
                }
            },
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
        } else if c == '\"' {
            advance(&mut chars, &mut line, &mut column);
            let token = read_string(&mut chars, &mut line, &mut column);
            tokens.push(token);
        } else if c == '\'' {
            advance(&mut chars, &mut line, &mut column);
            let token = read_char(&mut chars, &mut line, &mut column);
            tokens.push(token);
        } else {
            let token = read_operator_or_punctuation(&mut chars, &mut line, &mut column);
            tokens.push(token);
        }
    }

    tokens
}
