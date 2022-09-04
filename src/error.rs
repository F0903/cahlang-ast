use std::io::{stderr, Write};

use crate::token::{Token, TokenType};

pub fn report(line: usize, msg: &str) {
    stderr()
        .write_fmt(format_args!("{msg} at line {line}\n"))
        .ok();
}

pub fn error(token: Token, msg: &str) {
    report(token.line, msg);
}
