#[derive(Debug, PartialEq)]
pub enum ReservedWord {
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    Let,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Ellipsis,
    Semicolon,
    Comma,
    QuestionMark,
    Colon,
    DoubleColon,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    ExclamationMark,
    Ampersand,
    Pipe,
    Caret,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    DoubleEqual,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    PercentEqual,
    AmpersandEqual,
    PipeEqual,
    CaretEqual,
    LeftShiftEqual,
    RightShiftEqual,
    UnsignedRightShiftEqual,
    DoublePlus,
    DoubleMinus,
    Arrow,

    DoubleQuote,
    SingleQuote,
    TemplateQuote,
    DoubleQuoteStringWrapper,
    SingleQuoteStringWrapper,
    TemplateStringWrapper,
    Identifier(String),
    NumericLiteral(f64),
    StringLiteral(String),

    RegularExpressionLiteral(String),
    Template(String),
    LineTerminator,
    Whitespace,
    Comment,
    Error(String),
    Unknown,
    EOF
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer: Lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn get_current_char(&self) -> char {
        return match self.current_char {
            Some(curr) => curr,
            None => '\0',
        }
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
            if ch.is_digit(10) || ch == '.' {
                number.push(ch);
                self.advance();
            }
            else {
                break;
            }
        }

        if let Ok(parsed) = number.parse::<f64>() {
            Token::NumericLiteral(parsed)
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

    pub fn peek(&mut self) -> char {
        if self.position + 1 <= self.input.len() {
            return self.input[self.position + 1];
        }
        
        '\0'
    }

    pub fn reversePeek(&mut self) -> char {
        if self.position - 1 >= 0 {
            return self.input[self.position - 1];
        }
        
        '\0' 
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
                '(' => {
                    self.advance();
                    return Token::LeftParen;
                }
                ')' => {
                    self.advance();
                    return Token::RightParen;
                }
                '{' => {
                    self.advance();
                    return Token::LeftBrace;
                }
                '}' => {
                    self.advance();
                    return Token::RightBrace;
                }
                '[' => {
                    self.advance();
                    return Token::LeftBracket;
                }
                ']' => {
                    self.advance();
                    return Token::RightBracket;
                }
                '.' => {
                    self.advance();
                    if self.peek() == '.' {
                        self.advance();
                        if self.peek() == '.' {
                            self.advance();
                            return Token::Ellipsis;
                        }
                    }
                    return Token::Dot;
                }
                ';' => {
                    self.advance();
                    return Token::Semicolon;
                }
                ',' => {
                    self.advance();
                    return Token::Comma;
                }
                '?' => {
                    self.advance();
                    return Token::QuestionMark;
                }
                ':' => {
                    self.advance();
                    if self.peek() == ';' {
                        self.advance();
                        return Token::DoubleColon;
                    }
                    return Token::Colon;
                }
                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        if self.peek() == '=' {
                            self.advance();
                            return Token::StrictEqual;
                        }
                        return Token::DoubleEqual;
                    }
                    return Token::Equals;
                }
                '+' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::PlusEqual;
                    } else if self.peek() == '+' {
                        self.advance();
                        return Token::DoublePlus;
                    }

                    return Token::Plus;
                }
                '-' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::MinusEqual;
                    } else if self.peek() == '-' {
                        self.advance();
                        return Token::DoubleMinus;
                    }
                    return Token::Minus;
                }
                '*' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::AsteriskEqual;
                    }
                    return Token::Asterisk;
                }
                '/' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::SlashEqual;
                    }
                    return Token::Slash;
                }
                '%' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::PercentEqual;
                    }
                    return Token::Percent;
                }
                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        if self.peek() == '=' {
                            self.advance();
                            return Token::StrictNotEqual;
                        }
                        return Token::NotEqual;
                    }
                    return Token::ExclamationMark;
                }
                '&' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::AmpersandEqual;
                    }
                    return Token::Ampersand;
                }
                '|' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::PipeEqual;
                    }
                    return Token::Pipe;
                }
                '^' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        return Token::CaretEqual;
                    }
                    return Token::Caret;
                },
                '<' => {
                    self.advance();

                    if self.peek() == '=' {
                        self.advance();
                        return Token::LessThanOrEqual;
                    }

                    if self.peek() == '<' {
                        self.advance();
                        if self.peek() == '=' {
                            self.advance();
                            return Token::LeftShiftEqual;
                        }
                        return Token::NotEqual;
                    }
                    return Token::LessThan;
                }
                '>' => {
                    self.advance();

                    if self.peek() == '=' {
                        self.advance();
                        return Token::GreaterThanOrEqual;
                    }

                    if self.peek() == '>' {
                        self.advance();
                        if self.peek() == '=' {
                            self.advance();
                            return Token::RightShiftEqual;
                        }
                        return Token::NotEqual;
                    }
                    return Token::GreaterThan;
                },
                '"' => {
                    self.advance();
                    let mut final_value = String::new();

                    loop {
                        match self.peek() {
                            '"' => {
                                self.advance();
                                return Token::StringLiteral(final_value);
                            }
                            '\n' | '\r' => {
                                // Handle error: Unterminated string literal
                                return Token::Error("Unterminated string literal".to_string());
                            }
                            _ => {
                                final_value.push(self.get_current_char());
                                self.advance();
                            }
                        }
                    }
                }
                _ => panic!("Invalid token!")
            }
        }
        Token::EOF
    }
}