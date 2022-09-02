use std::any::Any;

use crate::token::Token;

pub enum Expression {
    Binary(Box<BinaryExpression>),
    Grouping(Box<GroupingExpression>),
    Literal(Box<LiteralExpression>),
    Unary(Box<UnaryExpression>),
}

pub struct BinaryExpression {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

pub struct GroupingExpression {
    pub expr: Expression,
}

pub struct LiteralExpression {
    pub value: Box<dyn Any>,
}

pub struct UnaryExpression {
    pub operator: Token,
    pub right: Expression,
}
