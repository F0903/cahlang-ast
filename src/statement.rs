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
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: BlockStatement,
    pub else_branch: Option<BlockStatement>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockStatement,
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Var(VarStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
}
