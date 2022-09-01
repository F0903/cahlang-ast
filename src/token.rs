use std::{any::Any, fmt::Display};
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single character
    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    BraceOpen,
    BraceClose,
    Dot,
    Comma,
    Equal,
    Less,
    Greater,
    Plus,
    Minus,
    Multiply,
    Divide,

    // Two characters
    EqualEqual,
    LessEqual,
    GreaterEqual,

    // Literals
    String,
    Number,
    Identifier,

    // Keywords
    Offering,
    Ritual,
    Return,
    Not,
    And,
    Or,
    Class,
    This,
    Super,
    While,
    For,
    If,
    Else,
    True,
    False,
    None,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>, //TODO: Maybe use a special enum for this with acceptable values.
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Box<dyn Any>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        ))
    }
}
