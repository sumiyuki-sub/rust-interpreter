#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    ExpressionStmt(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    BooleanLiteral(bool),
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
}
