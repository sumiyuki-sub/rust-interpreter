use crate::token::{Token, lookup_ident};

/// 入力文字列をトークン列に変換する字句解析器
pub struct Lexer {
    input: String,
    position: usize,      // 現在読んでいる位置
    read_position: usize, // 次に読む位置
    ch: u8,               // 現在の文字
}

impl Lexer {
    /// 新しいLexerを作る。最初の1文字を読み込んでおく
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    /// positionを1つ進めてchを更新する。read_positionあその1つ先を指す
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// 現在の文字からトークンを1つ読み取って返す
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            0 => Token::Eof,
            b'"' => {
                let s = self.read_string();
                return Token::StringLiteral(s);
            }
            c if is_letter(c) => {
                let ident = self.read_identifier();
                return lookup_ident(&ident);
            }
            c if is_digit(c) => {
                let number = self.read_number();
                return Token::Int(number.parse::<i64>().unwrap());
            }
            _ => Token::Illegal,
        };

        self.read_char();

        tok
    }

    /// 空白を読み飛ばす
    fn skip_whitespace(&mut self) {
        while matches!(self.ch, b' ' | b'\t' | b'\n' | b'\r') {
            self.read_char();
        }
    }

    /// 識別子を最後まで読む
    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[start..self.position].to_string()
    }

    /// 数字を最後まで読む
    fn read_number(&mut self) -> String {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[start..self.position].to_string()
    }

    /// 次の文字を先読みする（位置は進めない）
    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    /// 文字列リテラルを読む
    fn read_string(&mut self) -> String {
        self.read_char();
        let start = self.position;
        while self.ch != b'"' && self.ch != 0 {
            self.read_char();
        }
        let s = self.input[start..self.position].to_string();
        self.read_char();
        s
    }
}

fn is_letter(ch: u8) -> bool {
    (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch) || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    (b'0'..=b'9').contains(&ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(input: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(input.to_string());
        for (i, exp) in expected.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(&tok, exp, "test[{}] - tok wrong", i);
        }
    }

    #[test]
    fn next_token_with_symbols() {
        let input = "=+(){},;";

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        assert_tokens(input, expected);
    }

    #[test]
    fn next_token_with_full_program() {
        let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        }

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "foobar"
        "foo bar"
        "#;

        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::StringLiteral("foobar".to_string()),
            Token::StringLiteral("foo bar".to_string()),
            Token::Eof,
        ];

        assert_tokens(input, expected);
    }
}
