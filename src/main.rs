use std::io::{Read, Write, stdin, stdout};

mod database;
mod command;

fn main() {

    let db = database::Database::default();
    let mut parser = command::Parser::new();

    println!("]===================================[");
    println!("");
    println!("Welcome to rustdb!");
    println!("");
    println!("]===================================[");
    println!("> Enter a command or type 'HELP' to get an overview.");

    // Allocate a new string object to feed input into.
    let mut input = String::new();

    loop {
        // Print an arrow infront of the input line for asthetics.
        print!("> ");
        // Flush the output to present the arrow.
        stdout().flush().unwrap();

        // Read user input into input string buffer.
        stdin().read_line(&mut input).unwrap();

        // Drop last char from input line if it is either a newline (\n) or a carriage return escape character (\r).
        if let Some('\n') = input.chars().next_back(){
            input.pop();
        }
        if let Some('\r') = input.chars().next_back(){
            input.pop();
        }

        if input.eq_ignore_ascii_case("help"){
            print_help();
            continue;
        }

        // Match/switch command parse which returns Result<Command, ParseError>. 
        let command = match parser.parse(&input) {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("Failed to parse command: {}", err);
                continue;
            },
        };

        println!("Presented command: {}", command);


    }

}

fn print_help(){

}
