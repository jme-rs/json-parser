use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String), // 文字列
    Number(f64),    // 数値
    Bool(bool),     // boolean
    Null,           // null
    WhiteSpace,     // 空白
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Colon,          // :
}

/// 字句解析中のエラー
#[derive(Debug)]
pub struct LexerError {
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

/// 字句解析
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                Token::WhiteSpace => (),
                _ => tokens.push(token),
            }
        }

        Ok(tokens)
    }

    /// 1 文字進め、`Token` を返す
    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    /// 文字列から `Token` を返す
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.chars.peek() {
            Some(c) => match c {
                // 1 文字の token
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(self.next_return_token(Token::WhiteSpace))
                }
                '{' => Ok(self.next_return_token(Token::LeftBrace)),
                '}' => Ok(self.next_return_token(Token::RightBrace)),
                '[' => Ok(self.next_return_token(Token::LeftBracket)),
                ']' => Ok(self.next_return_token(Token::RightBracket)),
                ',' => Ok(self.next_return_token(Token::Comma)),
                ':' => Ok(self.next_return_token(Token::Colon)),

                // 複数文字の token
                // 文字列
                '"' => {
                    self.chars.next();
                    self.parse_string_token()
                }
                // 数値
                c if c.is_numeric() || matches!(c, '+' | '-' | '.') => self.parse_number_token(),
                // boolean
                't' => self.parse_bool_token(true),
                'f' => self.parse_bool_token(false),
                // null
                'n' => self.parse_null_token(),

                // その他
                _ => Err(LexerError::new(&format!(
                    "error: an unexpected char \"{}\"",
                    c
                ))),
            },
            None => Ok(None),
        }
    }

    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        let s = (0..4).filter_map(|_| self.chars.next()).collect::<String>();
        if s == "null" {
            Ok(Some(Token::Null))
        } else {
            Err(LexerError::new(&format!(
                "error: a null value is expected \"{}\"",
                s
            )))
        }
    }

    fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
        if b {
            let s = (0..4).filter_map(|_| self.chars.next()).collect::<String>();
            if s == "true" {
                Ok(Some(Token::Bool(true)))
            } else {
                Err(LexerError::new(&format!(
                    "error: a boolean true is expected \"{}\"",
                    s
                )))
            }
        } else {
            let s = (0..5).filter_map(|_| self.chars.next()).collect::<String>();
            if s == "false" {
                Ok(Some(Token::Bool(false)))
            } else {
                Err(LexerError::new(&format!(
                    "error: a boolean false is expected \"{}\"",
                    s
                )))
            }
        }
    }

    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut num_buf = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() || matches!(c, '+' | '-' | 'e' | 'E' | '.') {
                self.chars.next();
                num_buf.push(c);
            } else {
                break;
            }
        }
        match num_buf.parse::<f64>() {
            Ok(number) => Ok(Some(Token::Number(number))),
            Err(e) => Err(LexerError::new(&format!("error: {}", e.to_string()))),
        }
    }

    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut str_buf = String::new();
        let mut utf16_buf = vec![];

        while let Some(c1) = self.chars.peek() {
            match c1 {
                '\\' => {
                    let c2 = self
                        .chars
                        .next()
                        .ok_or(LexerError::new("error: a next char is expected"))?;
                    if matches!(c2, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') {}
                }
            }
        }
        todo!();
        fn push_utf16(str_buf: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
            if utf16.is_empty() {
                return Ok(());
            }
            match String::from_utf16(utf16) {
                Ok(utf16_str) => {
                    str_buf.push_str(&utf16_str);
                    utf16.clear();
                }
                Err(e) => {
                    return Err(LexerError::new(&format!("error: {}", e.to_string())));
                }
            };
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_token() {
        let s = "null";
        let tokens = Lexer::new(s).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Null);
    }

    #[test]
    fn bool_token() {
        let b = "true";
        let tokens = Lexer::new(b).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Bool(true));

        let b = "false";
        let tokens = Lexer::new(b).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Bool(false));
    }

    #[test]
    fn number_token() {
        // integer
        let num = "1234567890";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(1234567890f64));

        let num = "+123";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(123f64));

        // float
        let num = "-0.001";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(-0.001));

        let num = ".001";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(0.001));

        // exponent
        let num = "1e-10";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(0.0000000001));

        let num = "+2E10";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(20000000000f64));
    }
}
