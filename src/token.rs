#[derive(Debug, Clone, PartialEq)]

/// トークンの種類を表す
/// Lexerが文字列を分解した結果、Parserが受け取る単位
pub enum Token {
    /// 未対応の不正な文字
    Illegal,
    /// 入力の終端
    Eof,

    /// 識別子（変数名・関数名）例: "x", "add"
    Ident(String),
    /// 整数リテラル 例: 5, 42
    Int(i64),
    /// 文字列リテラル 例: "hello"
    StringLiteral(String),

    /// 演算子
    Assign, // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Lt,       // <
    Gt,       // >
    Eq,       // ==
    NotEq,    // !=

    /// 区切り文字
    Comma,
    Semicolon,
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    /// キーワード
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

/// 識別子文字列がキーワードかどうかを判定して対応するトークンを返す
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}
