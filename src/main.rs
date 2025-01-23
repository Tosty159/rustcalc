mod input;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    println!("RustCalc Alpha 2.0");
    println!("Input 'q' to terminate.");
    println!("\n");

    loop {
        let input = input::get_input();

        if let Some('q') = input.chars().next_back() {
            break;
        }

        let mut lexer = lexer::Lexer::new(&input);

        let mut parser = parser::Parser::new(&mut lexer);

        let ast = parser.parse();
        
        let result = interpreter::interpret(ast);

        println!("{result}");
    }
}
