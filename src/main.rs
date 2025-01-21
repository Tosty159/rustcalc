use core::panic;
use std::io::{stdout, Write};
use std::collections::HashSet;
use once_cell::sync::Lazy;
use termion::{raw::IntoRawMode, input::TermRead};

static ALLOWED: Lazy<HashSet<char>> = Lazy::new(|| {
    [' ', '\t','0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*', '/', '(', ')']
        .into_iter()
        .collect()
});

fn get_input() -> String {
    let stdin = std::io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut input = String::new();

    write!(stdout, "Calc> ").unwrap();
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.unwrap() {
            termion::event::Key::Char(c) if ALLOWED.contains(&c) => {
                input.push(c);
                write!(stdout, "{c}").unwrap();
                stdout.flush().unwrap();
            },
            termion::event::Key::Backspace => {
                if !input.is_empty() {
                    input.pop();
                    write!(stdout, "\u{8} \u{8}").unwrap();
                    stdout.flush().unwrap();
                }
            },
            termion::event::Key::Char('q') => {
                input.push('q');
                break;
            },
            termion::event::Key::Char('\n') => break,
            _ => {} // Ignore invalid keys
        }
    }
    println!();
    write!(stdout, "\r\x1B[K").unwrap();
    stdout.flush().unwrap();

    input
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Number(f64),
    Operator(char),
    LParen,
    RParen,
    EOF,
}

struct Lexer {
    source: String,
    position: usize,
}

impl Lexer {
    fn new(source: String) -> Self {
        Lexer {
            source,
            position: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        while self.position < self.source.len() {
            let ch = self.source.as_bytes()[self.position] as char;
            self.position += 1;

            return match ch {
                ' ' | '\t' => continue,
                '0'..='9' => self.tokenize_number(ch),
                '+' | '-' | '*' | '/' => Token::Operator(ch),
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => panic!("Unexpected character: {ch}"),
            };
        }
        Token::EOF
    }

    fn tokenize_number(&mut self, first_char: char) -> Token {
        let mut result_num = first_char.to_string();

        while self.position < self.source.len() {
            let ch = self.source.as_bytes()[self.position]  as char;

            if !(ch.is_numeric() || ch == '.') {
                break;
            }

            result_num.push(ch);
            self.position += 1;
        }

        Token::Number(result_num.parse().unwrap())
    }
}

#[derive(Debug)]
enum ASTNode {
    Number(f64),
    BinaryOperator {
        lhs: Box<ASTNode>,
        op: char,
        rhs: Box<ASTNode>
    },
    UnaryOperator {
        operand: Box<ASTNode>,
        op: char,
    },
}

struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn eat(&mut self, expected: Token) {
        if self.current_token == expected {
            self.advance();
        } else {
            panic!("Expected: {expected:?}, found: {:?}", self.current_token);
        }
    }

    fn parse(&mut self) -> ASTNode {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();

        loop {
            match self.current_token {
                Token::Operator(op) if op == '+' || op == '-' => {
                    self.advance();
                    let rhs = Box::new(self.parse_term());
                    node = ASTNode::BinaryOperator {
                        lhs: Box::new(node),
                        op,
                        rhs,
                    };
                },
                _ => break,
            }
        }

        node
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();

        loop {
            match self.current_token {
                Token::Operator(op) if op == '*' || op == '/' => {
                    self.advance();
                    let rhs = Box::new(self.parse_factor());
                    node = ASTNode::BinaryOperator {
                        lhs: Box::new(node),
                        op,
                        rhs,
                    };
                },
                _ => break,
            }
        }

        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        match self.current_token {
            Token::Number(num) => {
                self.advance();
                ASTNode::Number(num)
            },
            Token::Operator(op) if op == '+' || op == '-' => {
                self.advance();
                let operand = Box::new(self.parse_factor());
                ASTNode::UnaryOperator { operand, op }
            },
            Token::LParen => {
                self.advance();
                let node = self.parse_expression();
                self.eat(Token::RParen);
                node
            },
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }
}

fn interpret(ast: ASTNode) -> f64 {
    match ast {
        ASTNode::Number(num) => num,
        ASTNode::UnaryOperator { operand, op } => {
            let opr = interpret(*operand);
            match op {
                '+' => opr,
                '-' => -opr,
                _ => panic!("Invalid unary operator: {op}"),
            }
        },
        ASTNode::BinaryOperator { lhs, op, rhs } => {
            let left = interpret(*lhs);
            let right = interpret(*rhs);

            match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!("Invalid binary oeprator: {op}"),
            }
        }
    }
}

fn main() {
    println!("RustCalc Alpha 1.1");
    println!("Input 'q' to terminate.");
    println!("\n");

    loop {
        let input = get_input();

        if let Some('q') = input.chars().next_back() {
            break;
        }

        let mut lexer = Lexer::new(input.clone());

        let mut parser = Parser::new(&mut lexer);

        let ast = parser.parse();
        
        let result = interpret(ast);

        println!("{result}");
    }
}
