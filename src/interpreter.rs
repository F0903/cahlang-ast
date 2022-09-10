use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::{Env, Environment},
    error::{get_err_handler, Result},
    expression::{
        AssignExpression, BinaryExpression, Expression, LogicalExpression, PostfixExpression,
        UnaryExpression, VariableExpression,
    },
    statement::{
        BlockStatement, ExpressionStatement, IfStatement, PrintStatement, Statement, VarStatement,
        WhileStatement,
    },
    token::{Token, TokenType},
    value::Value,
};

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::None => false,
            Value::Boolean(x) => *x,
            _ => true,
        }
    }

    fn error<T>(token: Token, msg: impl ToString) -> Result<T> {
        Err((token, msg).into())
    }

    fn eval_unary(&mut self, expr: &UnaryExpression) -> Result<Value> {
        let right = self.evaluate(&expr.right)?;
        let val = match expr.operator.token_type {
            TokenType::Minus => {
                let val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
                            "Minus unary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Number(-val)
            }
            TokenType::Not => Value::Boolean(!Self::is_truthy(&right)),
            _ => {
                return Self::error(
                    expr.operator.clone(),
                    "Minus unary operator can only be used on numbers.",
                )
            }
        };
        Ok(val)
    }

    fn eval_postfix(&mut self, expr: &PostfixExpression) -> Result<Value> {
        let var = match &expr.left {
            Expression::Variable(x) => x,
            _ => {
                return Self::error(
                    expr.operator.clone(),
                    "Expected variable in postfix operator.",
                )
            }
        };
        let mut env = self.env.borrow_mut();
        let old_val = match env.get(&var.name)? {
            Value::Number(x) => x,
            _ => {
                return Self::error(
                    expr.operator.clone(),
                    "Postfix operators can only be used with numbers.",
                )
            }
        };
        let new_val = match expr.operator.token_type {
            TokenType::MinusMinus => Value::Number(old_val - 1.0),
            TokenType::PlusPlus => Value::Number(old_val + 1.0),
            _ => return Self::error(expr.operator.clone(), "Unrecognized postfix operator."),
        };
        env.assign(&var.name, new_val.clone())?;
        Ok(new_val)
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

    fn eval_binary(&mut self, expr: &BinaryExpression) -> Result<Value> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let val = match expr.operator.token_type {
            TokenType::Minus => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
                            "Minus binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
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
                            expr.operator.clone(),
                            "Divide binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
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
                            expr.operator.clone(),
                            "Multiply binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
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
                                    expr.operator.clone(),
                                    "Unknown right operand in string concat.",
                                )
                            }
                        }
                    }
                } else if let Value::Number(x) = left {
                    if let Value::Number(y) = right {
                        Value::Number(x + y)
                    } else {
                        return Self::error(
                            expr.operator.clone(),
                            "Cannot add non-number to number.",
                        );
                    }
                } else {
                    return Self::error(
                        expr.operator.clone(),
                        "Plus binary operator can only be used with strings or numbers",
                    );
                }
            }
            TokenType::Greater => {
                let left_val = match left {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
                            "Greater binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
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
                            expr.operator.clone(),
                            "Greater-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
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
                            expr.operator.clone(),
                            "Less binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
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
                            expr.operator.clone(),
                            "Less-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                let right_val = match right {
                    Value::Number(x) => x,
                    _ => {
                        return Self::error(
                            expr.operator.clone(),
                            "Less-or-Equal binary operator can only be used on numbers.",
                        )
                    }
                };
                Value::Boolean(left_val <= right_val)
            }
            TokenType::Is => Value::Boolean(Self::is_equal(left, right)),
            _ => {
                return Self::error(
                    expr.operator.clone(),
                    "Unknown operator in binary expression.",
                )
            }
        };
        Ok(val)
    }

    fn eval_variable(&self, expr: &VariableExpression) -> Result<Value> {
        self.env.borrow().get(&expr.name)
    }

    fn eval_assign(&mut self, expr: &AssignExpression) -> Result<Value> {
        let value = self.evaluate(&expr.value)?;
        self.env.borrow_mut().assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn eval_logical(&mut self, expr: &LogicalExpression) -> Result<Value> {
        let left = self.evaluate(&expr.left)?;
        if expr.operator.token_type == TokenType::Or {
            if Self::is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !Self::is_truthy(&left) {
                return Ok(left);
            }
        }
        Ok(self.evaluate(&expr.right)?)
    }

    fn evaluate(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Literal(x) => Ok(x.value.clone()),
            Expression::Grouping(x) => self.evaluate(&x.expr),
            Expression::Unary(x) => self.eval_unary(&*x),
            Expression::Postfix(x) => self.eval_postfix(&*x),
            Expression::Binary(x) => self.eval_binary(&*x),
            Expression::Variable(x) => self.eval_variable(&*x),
            Expression::Assign(x) => self.eval_assign(&*x),
            Expression::Logical(x) => self.eval_logical(&*x),
        }
    }

    fn execute_print_statement(&mut self, statement: &PrintStatement) -> Result<()> {
        let val = self.evaluate(&statement.expr)?;
        println!("{}", val);
        Ok(())
    }

    fn execute_expression_statement(&mut self, statement: &ExpressionStatement) -> Result<()> {
        self.evaluate(&statement.expr)?;
        Ok(())
    }

    fn execute_var_statement(&mut self, statement: &VarStatement) -> Result<()> {
        let mut value = Value::None;
        if let Some(init) = &statement.initializer {
            value = self.evaluate(&init)?;
        }
        self.env
            .borrow_mut()
            .define(statement.name.lexeme.clone(), value);
        Ok(())
    }

    fn execute_block(&mut self, statements: &[Statement], env: Env) -> Result<()> {
        let previous = self.env.clone();
        self.env = env;
        for statement in statements {
            self.execute(statement).ok();
        }
        self.env = previous;
        Ok(())
    }

    fn execute_block_statement(&mut self, statement: &BlockStatement) -> Result<()> {
        self.execute_block(
            &statement.statements,
            Rc::new(RefCell::new(Environment::new(Some(self.env.clone())))),
        )
    }

    fn execute_if_statement(&mut self, statement: &IfStatement) -> Result<()> {
        if Self::is_truthy(&self.evaluate(&statement.condition)?) {
            self.execute_block_statement(&statement.then_branch)?;
        } else if let Some(x) = &statement.else_branch {
            self.execute_block_statement(&x)?;
        }
        Ok(())
    }

    fn execute_while_statement(&mut self, statement: &WhileStatement) -> Result<()> {
        while Self::is_truthy(&self.evaluate(&statement.condition)?) {
            self.execute_block_statement(&statement.body)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Print(x) => self.execute_print_statement(x),
            Statement::Expression(x) => self.execute_expression_statement(x),
            Statement::Var(x) => self.execute_var_statement(x),
            Statement::Block(x) => self.execute_block_statement(x),
            Statement::If(x) => self.execute_if_statement(x),
            Statement::While(x) => self.execute_while_statement(x),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            if let Err(x) = self.execute(&statement) {
                get_err_handler().runtime_error(x);
            }
        }
    }
}
