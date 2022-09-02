use super::{token::Token, Result};
use crate::{create_string_map, report, token::TokenType};
use lazy_static::lazy_static;
use std::{any::Any, collections::HashMap};

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = create_string_map!(
        "and"      => TokenType::And,
        "class"    => TokenType::Class,
        "else"     => TokenType::Else,
        "false"    => TokenType::False,
        "true"     => TokenType::True,
        "for"      => TokenType::For,
        "if"       => TokenType::If,
        "none"     => TokenType::None,
        "or"       => TokenType::Or,
        "not"      => TokenType::Not,
        "return"   => TokenType::Return,
        "super"    => TokenType::Super,
        "this"     => TokenType::This,
        "offering" => TokenType::Offering,
        "ritual"   => TokenType::Ritual,
        "end"      => TokenType::End,
        "while"    => TokenType::While
    );
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
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
        let token = Token::new(token_type, text, Box::new(()), self.line);
        self.tokens.push(token);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Box<dyn Any>) {
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
            report(self.line, b"Unterminated string.");
            return;
        }

        // Closing "
        self.next_char();

        let literal = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token_literal(TokenType::String, Box::new(literal));
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

        let value = self.source[self.start..self.current].parse::<f64>();
        self.add_token_literal(TokenType::Number, Box::new(value));
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
            '\n' => self.line += 1,
            '(' => self.add_token(TokenType::ParenOpen),
            ')' => self.add_token(TokenType::ParenClose),
            '[' => self.add_token(TokenType::SquareOpen),
            ']' => self.add_token(TokenType::SquareClose),
            '{' => self.add_token(TokenType::BraceOpen),
            '}' => self.add_token(TokenType::BraceClose),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Multiply),
            '/' => self.add_token(TokenType::Divide),
            '=' => {
                let token = match self.matches_next('=') {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
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
        let chars = self.source.chars();
        while !self.at_end() {
            self.start = self.current;
            self.lex_token();
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            Box::new(()),
            self.line,
        ));
        Ok(self.tokens)
    }
}
