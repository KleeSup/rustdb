use std::{io::{Write, stdin, stdout}, println};

use crate::command::Command::{Delete, Get, Set};

mod database;
mod command;

fn main() {

    let mut db = database::Database::new();
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
        input.clear();
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

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("help"){
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

        //println!("=> Presented command: {}", command);

        

        match command {
            Get { key } => {
                let val = db.get(&key);
                match val {
                    Some(val) => println!("=> '{}' is '{}'", key, val),
                    None => println!("=> There is no value for key '{}' set!", key),
                };
            },
            Set { key, value } => {
                println!("=> '{}' was set to '{}'!", key, value);
                db.set(key, value);
            },
            Delete { key } => {
                let result = db.delete(&key);
                match result {
                    Some(_) => println!("=> Value for key '{}' was deleted!", key),
                    None => println!("=> Nothing to delete!") 
                }
            },
        };


    }

}

fn print_help(){
    println!("==> SET <key> <value>");
    println!("==> GET <key>");
    println!("==> DELETE <key>");
    println!("==> TYPE <key>");
    println!("");
}
