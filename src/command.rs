use std::{fmt::Display};
use crate::{database::Value};

#[derive(Debug)]
pub enum Command{
    Get {
        key: String,
    },
    Set {
        key: String,
        value: Value,
    },
    Delete {
        key: String,
    },
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get { key } => write!(f, "Get(key='{}')", key),
            Self::Set { key, value } => write!(f, "Set(key='{}',value='{}')", key, value),
            Self::Delete { key } => write!(f, "Delete(key='{}')", key),
        }
    }
}



#[derive(Debug)]
pub enum ParseError{
    InvalidCommand,
    MissingKey,
    MissingValue,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommand => write!(f, "Invalid command entered"),
            Self::MissingKey => write!(f, "Key missing"),
            Self::MissingValue => write!(f, "Value missing"),
        }
    }
}

pub struct Parser {
    buffer: String
}

impl Parser{
    pub fn new() -> Self{
        Self { buffer: String::with_capacity(256), }
    }

    pub fn parse(&mut self, text: &str) -> Result<Command, ParseError>{
        self.buffer.clear();
        self.buffer.push_str(text);
        self.buffer.make_ascii_lowercase();
        let mut parts = self.buffer.trim_start().split_whitespace();

        let command = match parts.next() {
            Some(cmd) => cmd,
            None => return Err(ParseError::InvalidCommand),
        };

        let key = match parts.next() {
                Some(key) => key,
                None => return Err(ParseError::MissingKey),
            };
        
        // Command: GET <key>
        if command.eq_ignore_ascii_case("get") {
            return Ok(Command::Get { key: key.to_string() });
        }

        // Command DELETE <key>
        if command.eq_ignore_ascii_case("delete") || command.eq_ignore_ascii_case("del") {
            return Ok(Command::Delete { key: key.to_string() });
        }

        // Command SET <key> <value>
        if command.eq_ignore_ascii_case("set"){
            let count = parts.clone().count();
            if count == 0{
                return Err(ParseError::MissingValue);
            }
            if count == 1{
                let val = match parts.next(){
                    Some(val) => val,
                    None => return Err(ParseError::MissingValue),
                };

                // String.parse() returns a Result<F, F::Err>. 
                // By writing String.parse::<i64>() we specify a generic type argument.
                // is_ok() on a Result<T, E> returns true if the value is Ok(...) and false if it is Err(...).  
                // if val.parse::<i64>().is_ok() {}
                //
                // However, since I want to keep the parsed result and not only check if it worked, this is better:
                if let Ok(number) = val.parse::<i64>(){
                    return Ok(Command::Set { key: key.to_string(), value: Value::Integer(number) });
                }else if let Ok(float) = val.parse::<f64>(){
                    return Ok(Command::Set { key: key.to_string(), value: Value::Float(float) });
                }else { // default to string
                    return Ok(Command::Set { key: key.to_string(), value: Value::String(val.to_string()) });
                }
            }else{ // list of values will be turned into a string list 
                let mut list = Vec::<String>::new();
                // iterate each remaining element and push it into the vector.
                loop {
                    let next = match parts.next(){
                        Some(val) => val,
                        None => break,
                    };
                    list.push(next.to_string());
                }
                return Ok(Command::Set { key: key.to_string(), value: Value::List(list) });
            }
        }

        return Err(ParseError::InvalidCommand);
    }

}