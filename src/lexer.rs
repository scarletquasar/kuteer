#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    Semicolon,
    EOF,
    Unknown,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    pub fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input[self.position]);
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    pub fn get_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if let Ok(parsed) = number.parse::<i64>() {
            Token::Number(parsed)
        } else {
            Token::Unknown
        }
    }

    fn get_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Token::Identifier(identifier)
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if ch.is_digit(10) {
                return self.get_number();
            }
            if ch.is_alphabetic() || ch == '_' {
                return self.get_identifier();
            }

            match ch {
                '+' => {
                    self.advance();
                    return Token::Plus;
                }
                '-' => {
                    self.advance();
                    return Token::Minus;
                }
                '*' => {
                    self.advance();
                    return Token::Asterisk;
                }
                '/' => {
                    self.advance();
                    return Token::Slash;
                }
                '=' => {
                    self.advance();
                    return Token::Equals;
                }
                ';' => {
                    self.advance();
                    return Token::Semicolon;
                }
                _ => {
                    self.advance();
                    return Token::Unknown
                },
            }
        }
        Token::EOF
    }
}