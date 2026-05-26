use std::vec;

use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
};

/// 演算子の優先度。数値が大きいほど優先度が高い（* > + など）
enum Precedence {
    Lowest,      // 0: デフォルト
    Equals,      // 1: ==, !=
    LessGreater, // 2: <, >
    Sum,         // 3: +, -
    Product,     // 4: *, /
    Call,        // 5: (
}

impl Precedence {
    fn value(&self) -> u8 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Call => 5,
        }
    }
}

/// トークンに対応する優先度を返す
fn token_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq | Token::NotEq => Precedence::Equals,
        Token::Lt | Token::Gt => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Asterisk | Token::Slash => Precedence::Product,
        Token::LParen => Precedence::Call,
        _ => Precedence::Lowest,
    }
}

/// トークン列をASTに変換する構文解析器
pub struct Parser {
    lexer: Lexer,
    current_token: Token, // 現在見ているトークン
    peek_token: Token,    // 1つ先読みしているトークン
    errors: Vec<String>,  // パースエラーを貯める（最初のエラーで止めない）
}

impl Parser {
    /// Lexerを受け取ってParserを初期化する。current_token/peek_tokenを1つずつ先に読み込んでおく
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

    /// エントリポイント。EOFまで文を読み続けてProgramを返す
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

    /// current_tokenを見てどのパーサーを呼ぶか振り分けるディスパッチャー
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
            value: Expression::IntegerLiteral(5), // TODO: 実際の式パースに置き換える
        })
    }

    /// peek_tokenが期待通りなら進めてtrue、違ったらエラーを貯めてfalse
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

        Some(Statement::Return(Expression::IntegerLiteral(5))) // TODO: 実際の式パースに置き換える
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Statement::ExpressionStmt(expr))
    }

    /// PrattParserの核心。prefixをパースしてから、優先度が高いinfixがあればループで取り込む
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
            Token::Function => self.parse_function_literal(),
            Token::True => Some(Expression::BooleanLiteral(true)),
            Token::False => Some(Expression::BooleanLiteral(false)),
            _ => None,
        }?;

        while self.peek_token != Token::Semicolon
            && precedence.value() < token_precedence(&self.peek_token).value()
        {
            self.next_token();
            left = match self.current_token {
                Token::LParen => self.parse_call_expression(left)?,
                _ => self.parse_infix_expression(left)?,
            };
        }

        Some(left)
    }

    /// 左辺を受け取って演算子と右辺をパースしInfixノードを返す
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let op = match &self.current_token {
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Eq => "==".to_string(),
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

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }

        let mut parameters = vec![];
        if self.peek_token != Token::RParen {
            self.next_token();
            if let Token::Ident(s) = &self.current_token {
                parameters.push(s.clone());
            }
            while self.peek_token == Token::Comma {
                self.next_token();
                self.next_token();
                if let Token::Ident(s) = &self.current_token {
                    parameters.push(s.clone());
                }
            }
        }
        if !self.expect_peek(Token::RParen) {
            return None;
        }

        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        self.next_token();
        let mut body = vec![];
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt)
            }
            self.next_token();
        }

        Some(Expression::FunctionLiteral { parameters, body })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let mut arguments = vec![];

        if self.peek_token != Token::RParen {
            self.next_token();

            arguments.push(self.parse_expression(Precedence::Lowest)?);

            while self.peek_token == Token::Comma {
                self.next_token();
                self.next_token();
                arguments.push(self.parse_expression(Precedence::Lowest)?);
            }
        }

        if !self.expect_peek(Token::RParen) {
            return None;
        }

        Some(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Program {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }

    #[test]
    fn let_statement() {
        let program = parse("let x = 5;");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Let {
                name: "x".to_string(),
                value: Expression::IntegerLiteral(5)
            }
        )
    }

    #[test]
    fn return_statement() {
        let program = parse("return 5;");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Return(Expression::IntegerLiteral(5))
        )
    }

    #[test]
    fn infix_expression() {
        let program = parse("5 + 3");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::ExpressionStmt(Expression::Infix {
                left: Box::new(Expression::IntegerLiteral(5)),
                operator: "+".to_string(),
                right: Box::new(Expression::IntegerLiteral(3)),
            })
        )
    }

    #[test]
    fn prefix_expression() {
        let program = parse("!true");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::ExpressionStmt(Expression::Prefix {
                operator: "!".to_string(),
                right: Box::new(Expression::BooleanLiteral(true))
            })
        )
    }

    #[test]
    fn if_expression() {
        let program = parse("if (x > 5) { return 5; }");

        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::ExpressionStmt(Expression::If {
                condition: Box::new(Expression::Infix {
                    left: Box::new(Expression::Identifier("x".to_string())),
                    operator: ">".to_string(),
                    right: Box::new(Expression::IntegerLiteral(5)),
                }),
                consequence: vec![Statement::Return(Expression::IntegerLiteral(5))],
                alternative: None,
            })
        )
    }

    #[test]
    fn function_expression() {
        let program = parse("fn(x, y) { x + y };");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::ExpressionStmt(Expression::FunctionLiteral {
                parameters: vec!["x".to_string(), "y".to_string()],
                body: vec![Statement::ExpressionStmt(Expression::Infix {
                    left: Box::new(Expression::Identifier("x".to_string())),
                    operator: "+".to_string(),
                    right: Box::new(Expression::Identifier("y".to_string())),
                })]
            })
        )
    }

    #[test]
    fn call_expression() {
        let program = parse("add(2, 3);");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::ExpressionStmt(Expression::Call {
                function: Box::new(Expression::Identifier("add".to_string())),
                arguments: vec![Expression::IntegerLiteral(2), Expression::IntegerLiteral(3)]
            })
        )
    }
}
