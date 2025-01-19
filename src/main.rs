use std::io::{stdin, stdout, Write};

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

    input
}

fn main() {
    println!("RustCalc Alpha 1.0");
    println!("Press Ctrl+c to terminate.");
    println!("\n");

    loop {
        let input = get_input();
    }
}
