mod ast_printer;
mod error;
mod expression;
mod interpreter;
mod lexer;
mod parser;
mod statement;
mod token;
mod utils;
mod value;

use ast_printer::print_ast;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::{
    env::args,
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_source_path() -> Option<String> {
    let mut args = args();
    args.nth(1)
}

fn run(source: String, interpreter: &mut Interpreter) -> Result<()> {
    println!("{}\n", source);
    let lexer = Lexer::new(source);
    let tokens = lexer.lex()?;

    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens.into_iter());
    let statements = parser.parse();
    interpreter.interpret(statements);
    Ok(())
}

fn run_interactively() -> Result<()> {
    let mut interpreter = Interpreter::new();
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
        run(strbuf.clone(), &mut interpreter).ok();
        strbuf.clear();
    }
    Ok(())
}

fn run_file(path: String) -> Result<()> {
    let mut interpreter = Interpreter::new();
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    run(buf, &mut interpreter)?;
    Ok(())
}

fn main() -> Result<()> {
    let debug_test = include_str!("../test.cah");
    run(debug_test.to_owned(), &mut Interpreter::new())
    /* match get_source_path() {
        Some(x) => run_file(x),
        None => run_interactively(),
    } */
}
