use crate::{token::Token, value::Value};

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

#[derive(Debug)]
pub struct GroupingExpression {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub value: Value,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Expression,
}

#[derive(Debug)]
pub struct VariableExpression {
    pub name: Token,
}

#[derive(Debug)]
pub struct AssignExpression {
    pub name: Token,
    pub value: Expression,
}

#[derive(Debug)]
pub struct LogicalExpression {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

// Look into not boxing the values?
#[derive(Debug)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Grouping(Box<GroupingExpression>),
    Literal(Box<LiteralExpression>),
    Unary(Box<UnaryExpression>),
    Variable(Box<VariableExpression>),
    Assign(Box<AssignExpression>),
    Logical(Box<LogicalExpression>),
}
