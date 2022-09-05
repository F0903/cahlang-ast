use crate::{expression::Expression, token::Token};

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct PrintStatement {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct VarStatement {
    pub name: Token,
    pub initializer: Option<Expression>,
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Var(VarStatement),
    Block(BlockStatement),
}
