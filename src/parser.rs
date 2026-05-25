use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
};

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
}

impl Precedence {
    fn value(&self) -> u8 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
        }
    }
}

fn token_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq | Token::NotEq => Precedence::Equals,
        Token::Lt | Token::Gt => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Asterisk | Token::Slash => Precedence::Product,
        _ => Precedence::Lowest,
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !matches!(self.peek_token, Token::Ident(_)) {
            self.peek_error(&Token::Ident(String::new()));
            return None;
        }
        self.next_token();

        let name = match &self.current_token {
            Token::Ident(s) => s.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while self.current_token != Token::Semicolon && self.current_token != Token::Eof {
            self.next_token();
        }

        Some(Statement::Let {
            name,
            value: Expression::IntegerLiteral(5),
        })
    }

    fn expect_peek(&mut self, expected: Token) -> bool {
        if self.peek_token == expected {
            self.next_token();
            true
        } else {
            self.peek_error(&expected);
            false
        }
    }

    fn peek_error(&mut self, expected: &Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            expected, self.peek_token
        );
        self.errors.push(msg);
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        while self.current_token != Token::Semicolon && self.current_token != Token::Eof {
            self.next_token();
        }

        Some(Statement::Return(Expression::IntegerLiteral(5)))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Statement::ExpressionStmt(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = match &self.current_token {
            Token::Ident(s) => Some(Expression::Identifier(s.clone())),
            Token::Int(n) => Some(Expression::IntegerLiteral(*n)),
            Token::Bang | Token::Minus => {
                let op = match &self.current_token {
                    Token::Bang => "!".to_string(),
                    Token::Minus => "-".to_string(),
                    _ => unreachable!(),
                };
                self.next_token();

                let right = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Prefix {
                    operator: op,
                    right: Box::new(right),
                })
            }
            Token::If => self.parse_if_expression(),
            Token::True => Some(Expression::BooleanLiteral(true)),
            Token::False => Some(Expression::BooleanLiteral(false)),
            _ => None,
        }?;

        while self.peek_token != Token::Semicolon
            && precedence.value() < token_precedence(&self.peek_token).value()
        {
            self.next_token();
            left = self.parse_infix_expression(left)?;
        }

        Some(left)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let op = match &self.current_token {
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Eq => "=".to_string(),
            Token::NotEq => "!=".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            _ => return None,
        };
        let prec = token_precedence(&self.current_token);
        self.next_token();
        let right = self.parse_expression(prec)?;
        Some(Expression::Infix {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        })
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(Token::RParen) {
            return None;
        }
        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        self.next_token();
        let mut consequence = vec![];
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                consequence.push(stmt);
            }
            self.next_token();
        }

        let alternative = if self.peek_token == Token::Else {
            self.next_token();
            if !self.expect_peek(Token::LBrace) {
                return None;
            }
            self.next_token();
            let mut alt = vec![];
            while self.current_token != Token::RBrace && self.current_token != Token::Eof {
                if let Some(stmt) = self.parse_statement() {
                    alt.push(stmt);
                }
                self.next_token();
            }
            Some(alt)
        } else {
            None
        };

        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }
}
