#[derive(Debug, Clone, PartialEq)]

/// プログラム全体。文のリストを持つ
pub struct Program {
    pub statements: Vec<Statement>,
}

// 文（Statement）：値を返さない処理の単位
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    ExpressionStmt(Expression), // 5 + 3;のような式だけの文。中身はExpression::Infix
}

/// 式（Expression）：値をもつ処理の単位
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    /// 前置演算子 例： !true, -5
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    /// 中置演算子 例： 5 + 3
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
    FunctionLiteral {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}
