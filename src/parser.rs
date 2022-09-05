use std::iter::Peekable;

use crate::{
    error::{get_err_handler, Result, RuntimeError},
    expression::{
        AssignExpression, BinaryExpression, Expression, GroupingExpression, LiteralExpression,
        UnaryExpression, VariableExpression,
    },
    statement::{BlockStatement, ExpressionStatement, PrintStatement, Statement, VarStatement},
    token::{Token, TokenType},
    value::Value,
};

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    last_token: Option<Token>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
            last_token: None,
        }
    }

    fn error<T>(token: &Token, msg: &str) -> Result<T> {
        get_err_handler().report(token.line, msg);
        Err(RuntimeError::new(token.clone(), msg))
    }

    fn check(&mut self, typ: TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek().token_type == typ
        }
    }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn previous(&self) -> Token {
        self.last_token.clone().unwrap()
    }

    fn advance(&mut self) -> Token {
        let ret = self.peek().clone();
        let next = self.tokens.next().unwrap();
        self.last_token = Some(next);
        return ret;
    }

    fn match_next(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.check(*typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.at_end() {
            if self.previous().token_type == TokenType::StatementEnd {
                return;
            }

            // Skip through untill a statement end is hit, then advance.
            if self.peek().token_type == TokenType::StatementEnd {
                self.advance();
                return;
            }
        }
    }

    fn consume_if(&mut self, token_type: TokenType, err_msg: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Self::error(self.peek(), err_msg)
    }

    fn handle_primary(&mut self) -> Result<Expression> {
        if self.match_next(&[TokenType::Identifier]) {
            Ok(Expression::Variable(Box::new(VariableExpression {
                name: self.previous(),
            })))
        } else if self.match_next(&[TokenType::False]) {
            Ok(Expression::Literal(Box::new(LiteralExpression {
                value: Value::Boolean(false),
            })))
        } else if self.match_next(&[TokenType::True]) {
            Ok(Expression::Literal(Box::new(LiteralExpression {
                value: Value::Boolean(true),
            })))
        } else if self.match_next(&[TokenType::None]) {
            Ok(Expression::Literal(Box::new(LiteralExpression {
                value: Value::None,
            })))
        } else if self.match_next(&[TokenType::Number, TokenType::String]) {
            Ok(Expression::Literal(Box::new(LiteralExpression {
                value: self.previous().literal,
            })))
        } else if self.match_next(&[TokenType::ParenOpen]) {
            let expr = self.handle_expression()?;
            self.consume_if(TokenType::ParenClose, "Expected ')' after expression.")?;
            Ok(Expression::Grouping(Box::new(GroupingExpression { expr })))
        } else {
            Self::error(self.peek(), "Expected an expression.")
        }
    }

    fn handle_unary(&mut self) -> Result<Expression> {
        if self.match_next(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.handle_unary()?;
            return Ok(Expression::Unary(Box::new(UnaryExpression {
                operator,
                right,
            })));
        }
        self.handle_primary()
    }

    fn handle_factor(&mut self) -> Result<Expression> {
        let mut expr = self.handle_unary()?;
        while self.match_next(&[TokenType::Divide, TokenType::Multiply]) {
            let operator = self.previous();
            let right = self.handle_unary()?;
            expr = Expression::Binary(Box::new(BinaryExpression {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    fn handle_term(&mut self) -> Result<Expression> {
        let mut expr = self.handle_factor()?;
        while self.match_next(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.handle_factor()?;
            expr = Expression::Binary(Box::new(BinaryExpression {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    fn handle_comparison(&mut self) -> Result<Expression> {
        let mut expr = self.handle_term()?;
        while self.match_next(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.handle_term()?;
            expr = Expression::Binary(Box::new(BinaryExpression {
                left: expr,
                operator: operator.clone(),
                right,
            }));
        }
        Ok(expr)
    }

    fn handle_equality(&mut self) -> Result<Expression> {
        let mut expr = self.handle_comparison()?;
        while self.match_next(&[TokenType::Is, TokenType::Not]) {
            let operator = self.previous();
            let right = self.handle_comparison()?;
            expr = Expression::Binary(Box::new(BinaryExpression {
                left: expr,
                operator: operator.clone(),
                right,
            }));
        }
        Ok(expr)
    }

    fn handle_assignment(&mut self) -> Result<Expression> {
        let expr = self.handle_equality()?;
        if self.match_next(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.handle_assignment()?;
            if let Expression::Variable(x) = expr {
                let name = x.name;
                return Ok(Expression::Assign(Box::new(AssignExpression {
                    name,
                    value,
                })));
            }

            // Dont throw, just report
            Self::error::<RuntimeError>(&equals, "Invalid assignment target.").ok();
        }
        Ok(expr)
    }

    fn handle_expression(&mut self) -> Result<Expression> {
        self.handle_assignment()
    }

    fn handle_print_statement(&mut self) -> Result<Statement> {
        let expr = self.handle_expression()?;
        self.consume_if(
            TokenType::StatementEnd,
            "Expected statement end after expression.",
        )?;
        Ok(Statement::Print(PrintStatement { expr }))
    }

    fn handle_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.handle_expression()?;
        self.consume_if(
            TokenType::StatementEnd,
            "Expected statement end after expression.",
        )?;
        Ok(Statement::Expression(ExpressionStatement { expr }))
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = vec![];
        while !self.check(TokenType::BraceClose) && !self.at_end() {
            statements.push(self.handle_declaration()?);
        }
        self.consume_if(TokenType::BraceClose, "Expected '}' after block.")?;
        self.consume_if(
            TokenType::StatementEnd,
            "Expected statement end after block.",
        )?;
        Ok(statements)
    }

    fn handle_block_statement(&mut self) -> Result<Statement> {
        Ok(Statement::Block(BlockStatement {
            statements: self.parse_block()?,
        }))
    }

    fn handle_statement(&mut self) -> Result<Statement> {
        if self.match_next(&[TokenType::DollarLess]) {
            return self.handle_print_statement();
        } else if self.match_next(&[TokenType::BraceOpen]) {
            return self.handle_block_statement();
        }
        self.handle_expression_statement()
    }

    fn handle_var_declaration(&mut self) -> Result<Statement> {
        let name = self.consume_if(TokenType::Identifier, "Expected variable name.")?;
        let mut initializer = None;
        if self.match_next(&[TokenType::Equal]) {
            initializer = Some(self.handle_expression()?);
        }
        self.consume_if(
            TokenType::StatementEnd,
            "Expected statement end after variable declaration.",
        )?;
        Ok(Statement::Var(VarStatement { name, initializer }))
    }

    fn handle_declaration(&mut self) -> Result<Statement> {
        let had_err;
        if self.match_next(&[TokenType::Offering]) {
            match self.handle_var_declaration() {
                Ok(x) => return Ok(x),
                Err(_) => had_err = true,
            }
        } else {
            match self.handle_statement() {
                Ok(x) => return Ok(x),
                Err(_) => had_err = true,
            }
        }

        if had_err {
            self.synchronize();
            // Dont throw error, so just return a None expr statement
            return Ok(Statement::Expression(ExpressionStatement {
                expr: Expression::Literal(Box::new(LiteralExpression { value: Value::None })),
            }));
        }

        Self::error(self.peek(), "Unexpected token in declaration.")
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        while !self.at_end() {
            let decl_statement = match self.handle_declaration() {
                Ok(x) => x,
                Err(_) => todo!(),
            };
            statements.push(decl_statement);
        }
        statements
    }
}
