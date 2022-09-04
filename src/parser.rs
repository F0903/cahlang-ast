use std::{error::Error, fmt::Display, iter::Peekable};

use crate::{
    error::report,
    expression::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression,
    },
    token::{Token, TokenType, Value},
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for ParseError {}

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

    fn error(token: &Token, msg: &str) -> ParseError {
        report(token.line, msg);
        ParseError::new(msg)
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
        let mut ret = self.last_token.clone();
        let next = self.tokens.next().unwrap();
        if let None = ret {
            ret = Some(next.clone());
        }
        self.last_token = Some(next);
        return unsafe { ret.unwrap_unchecked() };
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

    fn consume_if(&mut self, token_type: TokenType, err_msg: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
        }

        Err(Self::error(self.peek(), err_msg))
    }

    fn handle_primary(&mut self) -> Result<Expression> {
        if self.match_next(&[TokenType::False]) {
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
            Err(Self::error(self.peek(), "Expected an expression."))
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

    fn handle_expression(&mut self) -> Result<Expression> {
        self.handle_equality()
    }

    pub fn parse(&mut self) -> Expression {
        match self.handle_expression() {
            Ok(x) => x,
            Err(_) => todo!(),
        }
    }
}
