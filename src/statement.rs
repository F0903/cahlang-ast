use crate::expression::Expression;

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct PrintStatement {
    pub expr: Expression,
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
}
