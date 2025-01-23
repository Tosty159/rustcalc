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
    UnaryOperator(char),
    LParen,
    RParen,
    EOF,
}

struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current_char: Option<char>,
    last_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
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

    fn next_token(&mut self) -> Token {
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
                '*' | '/' => {
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

#[derive(Debug)]
enum ASTNode {
    Number(f64),
    BinaryOperator {
        lhs: Box<ASTNode>,
        op: char,
        rhs: Box<ASTNode>
    },
    UnaryOperator {
        op: char,
        operand: Box<ASTNode>,
    },
}

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Self {
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

    fn precedence(&mut self, tkn: Token) -> u16 {
        match tkn {
            Token::Operator(op) => {
                match op {
                    '+' | '-' => 1,
                    '*' | '/' => 2,
                    '^' => 3,
                    _ => 0,
                }
            },
            Token::UnaryOperator(_) => 10,
            _ => 0,
        }
    }

    fn is_left_associative(&mut self, tkn: Token) -> bool {
        match tkn {
            Token::Operator(op) => {
                match op {
                    '+' | '-' | '*' | '/' => true,
                    '^' => false,
                    _ => panic!("Invalid operator: {op}"),
                }
            },
            Token::UnaryOperator(_) => false,
            _ => panic!("Not an operator: {tkn:?}"),
        }
    }

    fn shunting_yard(&mut self) -> Vec<Token> {
        let mut output_queue: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        while self.current_token != Token::EOF {
            match self.current_token {
                Token::Number(_) => output_queue.push(self.current_token),
                Token::Operator(_) => {
                    while let Some(&top_op) = operator_stack.last() {
                        if matches!(top_op, Token::Operator(_) | Token::UnaryOperator(_)) {
                            let op_precedence = self.precedence(self.current_token);
                            let top_precedence = self.precedence(top_op);

                            if top_precedence > op_precedence || (top_precedence == op_precedence && self.is_left_associative(top_op)) {
                                output_queue.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(self.current_token);
                },
                Token::UnaryOperator(_) => operator_stack.push(self.current_token),
                Token::LParen => operator_stack.push(self.current_token),
                Token::RParen => {
                    while let Some(&op) = operator_stack.last() {
                        if op == Token::LParen {
                            break;
                        }

                        output_queue.push(operator_stack.pop().unwrap());
                    }
                    if !matches!(operator_stack.pop(), Some(Token::LParen)) {
                        panic!("Mismatched parentheses");
                    }
                },
                Token::EOF => break,
            }
            self.advance();
        }
        while let Some(&op) = operator_stack.last() {
            if op == Token::LParen {
                panic!("Mismatched parentheses");
            }
            output_queue.push(operator_stack.pop().unwrap());
        }

        output_queue
    }

    fn rpn_to_ast(&mut self, rpn_tokens: Vec<Token>) -> Option<ASTNode> {
        let mut stack: Vec<ASTNode> = Vec::new();

        for token in rpn_tokens {
            match token {
                Token::Number(num) => stack.push(ASTNode::Number(num)),
                Token::Operator(op) => {
                    if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
                        stack.push(ASTNode::BinaryOperator {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                        });
                    } else {
                        return None;
                    }
                },
                Token::UnaryOperator(op) => {
                    if let Some(opr) = stack.pop() {
                        stack.push(ASTNode::UnaryOperator {
                            op,
                            operand: Box::new(opr),
                        });
                    } else {
                        return None;
                    }
                }
                _ => {},
            }
        }

        if stack.len() == 1 {
            Some(stack.pop().unwrap())
        } else {
            None // Malformed RPN
        }
    }

    fn parse(&mut self) -> ASTNode {
        let output_queue = self.shunting_yard();
        match self.rpn_to_ast(output_queue) {
            Some(node) => node,
            None => panic!("Malformed expression"),
        }
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

        let mut lexer = Lexer::new(&input);

        let mut parser = Parser::new(&mut lexer);

        let ast = parser.parse();
        
        let result = interpret(ast);

        println!("{result}");
    }
}
