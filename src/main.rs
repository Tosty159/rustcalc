use std::io::{stdin, stdout, Write};

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

fn main() {
    println!("RustCalc Alpha 1.0");
    println!("Press Ctrl+c to terminate.");
    println!("\n");

    loop {
        let input = get_input();
    }
}
