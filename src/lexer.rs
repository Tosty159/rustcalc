#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(char),
    UnaryOperator(char),
    LParen,
    RParen,
    EOF,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current_char: Option<char>,
    last_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars().peekable();
        let current_char = chars.next();
        Lexer {
            chars,
            current_char,
            last_token: None,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            self.advance();

            return match ch {
                ' ' | '\t' => continue,
                '0'..='9' => {
                    self.last_token = Some(Token::Number(0.0));
                    self.tokenize_number(ch)
                },
                '+' | '-' => {
                    let token = if self.last_token.is_none() 
                        || matches!(self.last_token, Some(Token::Operator(_)) | Some(Token::LParen))
                    {
                        Token::UnaryOperator(ch)
                    } else {
                        Token::Operator(ch)
                    };
                    self.last_token = Some(token);
                    token
                },
                '*' | '/' | '^' => {
                    self.last_token = Some(Token::Operator(ch));
                    Token::Operator(ch)
                },
                '(' => {
                    self.last_token = Some(Token::LParen);
                    Token::LParen
                },
                ')' => {
                    self.last_token = Some(Token::RParen);
                    Token::RParen
                },
                _ => panic!("Unexpected character: {ch}"),
            };
        }
        Token::EOF
    }

    fn tokenize_number(&mut self, first_char: char) -> Token {
        let mut result_num = first_char.to_string();

        while let Some(ch) = self.current_char {

            if !(ch.is_numeric() || ch == '.') {
                break;
            }

            result_num.push(ch);
            self.advance();
        }

        Token::Number(result_num.parse().unwrap())
    }
}