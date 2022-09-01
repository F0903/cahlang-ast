mod lexer;
mod token;
mod utils;

use lexer::Lexer;
use std::{
    env::args,
    fs::File,
    io::{stderr, stdin, stdout, BufRead, Read, Write},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn report(line: usize, msg: &[u8]) {
    let msg = unsafe { std::str::from_utf8_unchecked(msg) };
    stderr()
        .write_fmt(format_args!("{msg} at line {line}"))
        .ok();
}

fn get_source_path() -> Option<String> {
    let mut args = args();
    args.nth(1)
}

fn run(source: String) -> Result<()> {
    let lexer = Lexer::new(source);
    let tokens = lexer.lex()?;

    for token in tokens {
        println!("{token}");
    }

    Ok(())
}

fn run_interactively() -> Result<()> {
    let mut stdout = stdout().lock();
    let mut stdin = stdin().lock();
    let mut strbuf = String::new();
    loop {
        stdout.write_all(b"> ")?;
        stdout.flush()?;
        let count = stdin.read_line(&mut strbuf)?;
        if count == 0 {
            break;
        }
        run(strbuf.clone()).ok();
        strbuf.clear();
    }
    Ok(())
}

fn run_file(path: String) -> Result<()> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    run(buf)?;
    Ok(())
}

fn main() -> Result<()> {
    match get_source_path() {
        Some(x) => run_file(x),
        None => run_interactively(),
    }
}
