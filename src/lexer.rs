use super::{token::Token, Result};
use crate::{create_string_map, error::get_err_handler, token::TokenType, value::Value};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    create_string_map!(
        "and"      => TokenType::And,
        "class"    => TokenType::Class,
        "else"     => TokenType::Else,
        "false"    => TokenType::False,
        "true"     => TokenType::True,
        "for"      => TokenType::For,
        "if"       => TokenType::If,
        "none"     => TokenType::None,
        "or"       => TokenType::Or,
        "is"       => TokenType::Is,
        "not"      => TokenType::Not,
        "return"   => TokenType::Return,
        "super"    => TokenType::Super,
        "this"     => TokenType::This,
        "offering" => TokenType::Offering,
        "ritual"   => TokenType::Ritual,
        "end"      => TokenType::End,
        "while"    => TokenType::While
    )
});

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    ignore_newline: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            ignore_newline: false,
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn get_current_char_unchecked(&self) -> char {
        unsafe { self.source.chars().nth(self.current).unwrap_unchecked() }
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.get_current_char_unchecked()
        }
    }

    fn peekpeek(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            unsafe { self.source.chars().nth(self.current + 1).unwrap_unchecked() }
        }
    }

    fn next_char(&mut self) -> char {
        let ch = self.get_current_char_unchecked();
        self.current += 1;
        ch
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_owned();
        let token = Token::new(token_type, text, Value::None, self.line);
        self.tokens.push(token);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Value) {
        let text = self.source[self.start..self.current].to_owned();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }

    fn matches_next(&mut self, ch: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.peek() != ch {
            return false;
        }

        self.current += 1;
        return true;
    }

    const fn alphanumeric_or_underscore(ch: char) -> bool {
        ch == '_' || ch.is_ascii_alphanumeric()
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next_char();
        }

        if self.at_end() {
            get_err_handler().report(self.line, "Unterminated string.");
            return;
        }

        // Closing "
        self.next_char();

        let literal = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token_literal(TokenType::String, Value::String(literal));
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.next_char();
        }

        if self.peek() == '.' && self.peekpeek().is_ascii_digit() {
            // Consume the .
            self.next_char();
            while self.peek().is_ascii_digit() {
                self.next_char();
            }
        }

        let value = match self.source[self.start..self.current].parse::<f64>() {
            Ok(x) => x,
            Err(_) => {
                get_err_handler().report(self.line, "Could not parse number!");
                return;
            }
        };
        self.add_token_literal(TokenType::Number, Value::Number(value));
    }

    fn handle_identifier(&mut self) {
        while Self::alphanumeric_or_underscore(self.peek()) {
            self.next_char();
        }

        let text = self.source[self.start..self.current].to_owned();
        let token_type = match KEYWORDS.get(&text) {
            Some(x) => *x,
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    fn is_maybe_stmt_end(test_type: &TokenType) -> bool {
        static STMT_END_TOKENS: &'static [TokenType] = &[
            TokenType::BraceClose,
            TokenType::ParenClose,
            TokenType::SquareClose,
            TokenType::True,
            TokenType::False,
            TokenType::Number,
            TokenType::String,
            TokenType::None,
            TokenType::End,
            TokenType::Identifier,
        ];
        STMT_END_TOKENS.iter().any(|x| x == test_type)
    }

    fn lex_token(&mut self) {
        let next = self.next_char();
        match next {
            '?' => {
                // Skip line, is a comment.
                while self.peek() != '\n' && !self.at_end() {
                    self.next_char();
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                if !self.ignore_newline {
                    let len = self.tokens.len();
                    if len == 0 {
                        return;
                    }
                    let last = match self.tokens.get(len - 1) {
                        Some(x) => x,
                        None => return,
                    };
                    if Self::is_maybe_stmt_end(&last.token_type) {
                        self.add_token(TokenType::StatementEnd);
                    }
                }
            }
            '(' => {
                self.add_token(TokenType::ParenOpen);
                self.ignore_newline = true;
            }
            ')' => {
                self.add_token(TokenType::ParenClose);
                self.ignore_newline = false;
            }
            '[' => {
                self.add_token(TokenType::SquareOpen);
                self.ignore_newline = true;
            }
            ']' => {
                self.add_token(TokenType::SquareClose);
                self.ignore_newline = false;
            }
            '{' => {
                self.add_token(TokenType::BraceOpen);
            }
            '}' => self.add_token(TokenType::BraceClose),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => {
                if self.matches_next('-') {
                    self.add_token(TokenType::MinusMinus)
                } else if self.matches_next('=') {
                    self.add_token(TokenType::MinusEqual)
                } else {
                    self.add_token(TokenType::Minus)
                }
            }
            '+' => {
                if self.matches_next('+') {
                    self.add_token(TokenType::PlusPlus)
                } else if self.matches_next('=') {
                    self.add_token(TokenType::PlusEqual)
                } else {
                    self.add_token(TokenType::Plus)
                }
            }
            '*' => self.add_token(TokenType::Multiply),
            '/' => self.add_token(TokenType::Divide),
            '=' => self.add_token(TokenType::Equal),
            '$' => {
                let token = if self.matches_next('>') {
                    TokenType::DollarGreater
                } else if self.matches_next('<') {
                    TokenType::DollarLess
                } else {
                    return;
                };
                self.add_token(token);
            }
            '<' => {
                let token = match self.matches_next('=') {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                };
                self.add_token(token);
            }
            '>' => {
                let token = match self.matches_next('=') {
                    true => TokenType::GreaterEqual,
                    false => TokenType::Greater,
                };
                self.add_token(token);
            }
            '"' => self.handle_string(),
            _ => {
                if next.is_ascii_digit() {
                    self.handle_number();
                } else if next.is_ascii_alphanumeric() {
                    self.handle_identifier();
                }
            }
        }
    }

    //TODO: Convert to iterator
    pub fn lex(mut self) -> Result<Vec<Token>> {
        while !self.at_end() {
            self.start = self.current;
            self.lex_token();
        }
        self.tokens.push(Token::new(
            TokenType::StatementEnd,
            "StatementEnd".to_owned(),
            Value::None,
            self.line,
        ));
        self.tokens.push(Token::new(
            TokenType::EOF,
            "EOF".to_owned(),
            Value::None,
            self.line,
        ));
        Ok(self.tokens)
    }
}
