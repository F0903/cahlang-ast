use crate::{
    error::{get_err_handler, Result},
    expression::{BinaryExpression, Expression, UnaryExpression},
    token::{Token, TokenType},
    value::Value,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn is_truthy(val: Value) -> bool {
        match val {
            Value::None => false,
            Value::Boolean(x) => x,
            _ => true,
        }
    }

    fn error<T>(token: Token, msg: impl ToString) -> Result<T> {
        Err((token, msg).into())
    }

    fn eval_unary(&self, expr: UnaryExpression) -> Result<Value> {
        let right = self.evaluate(expr.right)?;
        let val = match expr.operator.token_type {
            TokenType::Minus => {
                let val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Minus unary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Number(-val)
            }
            TokenType::Not => Value::Boolean(!Self::is_truthy(right)),
            _ => {
                return Self::error(
                    expr.operator,
                    "Minus unary operator can only be used on numbers.",
                )
            }
        };
        Ok(val)
    }

    fn is_equal(a: Value, b: Value) -> bool {
        match a {
            Value::None => match b {
                Value::None => true,
                _ => false,
            },
            Value::Boolean(x) => match b {
                Value::Boolean(y) => y == x,
                _ => false,
            },
            Value::Number(x) => match b {
                Value::Number(y) => x == y,
                _ => false,
            },
            Value::String(x) => match b {
                Value::String(y) => x == y,
                _ => false,
            },
        }
    }

    fn eval_binary(&self, expr: BinaryExpression) -> Result<Value> {
        let left = self.evaluate(expr.left)?;
        let right = self.evaluate(expr.right)?;
        let val = match expr.operator.token_type {
            TokenType::Minus => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Minus binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Minus binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Number(left_val - right_val)
            }
            TokenType::Divide => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Divide binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Divide binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Number(left_val / right_val)
            }
            TokenType::Multiply => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Multiply binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Multiply binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Number(left_val * right_val)
            }
            TokenType::Plus => {
                if let Value::String(x) = left {
                    if let Value::String(y) = right {
                        Value::String(x + &y)
                    } else {
                        match right {
                            Value::Number(y) => Value::String(x + &y.to_string()),
                            Value::Boolean(y) => Value::String(x + &y.to_string()),
                            Value::None => Value::String(x + "none"),
                            _ => {
                                return Self::error(
                                    expr.operator,
                                    "Unknown right operand in string concat.",
                                )
                            }
                        }
                    }
                } else if let Value::Number(x) = left {
                    if let Value::Number(y) = right {
                        Value::Number(x + y)
                    } else {
                        return Self::error(expr.operator, "Cannot add non-number to number.");
                    }
                } else {
                    return Self::error(
                        expr.operator,
                        "Plus binary operator can only be used with strings or numbers",
                    );
                }
            }
            TokenType::Greater => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Greater binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Greater binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Boolean(left_val > right_val)
            }
            TokenType::GreaterEqual => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Greater-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Greater-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Boolean(left_val >= right_val)
            }
            TokenType::Less => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Less binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Less binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Boolean(left_val < right_val)
            }
            TokenType::LessEqual => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Less-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator,
                            "Less-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Boolean(left_val <= right_val)
            }
            TokenType::Is => Value::Boolean(Self::is_equal(left, right)),
            _ => return Self::error(expr.operator, "Unknown operator in binary expression."),
        };
        Ok(val)
    }

    fn evaluate(&self, expr: Expression) -> Result<Value> {
        match expr {
            Expression::Literal(x) => Ok(x.value),
            Expression::Grouping(x) => self.evaluate(x.expr),
            Expression::Unary(x) => self.eval_unary(*x),
            Expression::Binary(x) => self.eval_binary(*x),
        }
    }

    pub fn interpret(&self, expression: Expression) {
        match self.evaluate(expression) {
            Ok(x) => println!("{}", x),
            Err(x) => get_err_handler().runtime_error(x),
        }
    }
}
