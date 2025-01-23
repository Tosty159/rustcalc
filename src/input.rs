use std::io::{stdout, Write};
use std::collections::HashSet;
use once_cell::sync::Lazy;
use termion::{raw::IntoRawMode, input::TermRead};

static ALLOWED: Lazy<HashSet<char>> = Lazy::new(|| {
    [' ', '\t','0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*', '/', '^', '(', ')']
        .into_iter()
        .collect()
});

pub fn get_input() -> String {
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