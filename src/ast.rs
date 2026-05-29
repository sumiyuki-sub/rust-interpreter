#[derive(Debug, Clone, PartialEq)]

/// プログラム全体。文のリストを持つ
pub struct Program {
    pub statements: Vec<Statement>,
}

/// 文（Statement）：値を返さない処理の単位
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    ExpressionStmt(Expression), // 5 + 3;のような式だけの文。中身はExpression::Infix
}

/// 式（Expression）：値をもつ処理の単位
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// 変数名 例: x, add
    Identifier(String),
    /// 整数リテラル 例: 5, 42
    IntegerLiteral(i64),
    StringLiteral(String),
    /// 真偽値リテラル 例: true, false
    BooleanLiteral(bool),
    /// 前置演算子 例: !true, -5
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    /// 中置演算子 例: 5 + 3
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    /// if式 例: if (x > 5) { x } else { 5 }
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    /// 関数リテラル 例: fn(x, y) { x + y }
    FunctionLiteral {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    /// 関数呼び出し 例: add(1, 2)
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}
