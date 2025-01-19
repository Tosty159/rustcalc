use std::{io::{stdin, stdout, Write}, result};

fn check_syntax(s: String) -> String {
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if !(ch.is_whitespace() || ch.is_numeric() || ['+', '-', '*', '/', '.', '(', ')'].contains(&ch)) {
            panic!("Unexpected character: {ch}");
        }
    }

    s
}

fn get_input() -> String {
    let mut input = String::new();

    print!("Calc> ");
    stdout().flush().unwrap();

    stdin().read_line(&mut input).expect("That's not a valid input...");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    check_syntax(input)
}

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

fn main() {
    println!("RustCalc Alpha 1.0");
    println!("Press Ctrl+c to terminate.");
    println!("\n");

    loop {
        let input = get_input();

        let lexer = Lexer::new(input);
    }
}
