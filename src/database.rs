use std::collections::HashMap;
use std::fmt::Display;
use std::write;

#[derive(Debug)]
pub enum Value{
    String(String),
    Integer(i64),
    Float(f64),
    List(Vec<String>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::String(s) => write!(f, "{}", s),
            Self::Integer(int) => write!(f, "{}", int),
            Self::Float(float) => write!(f, "{}", float),
            Self::List(array) => write!(f, "{:?}", array),
        }
    }
}

#[derive(Default)]
pub struct Database{
    entries: HashMap<String, Value>
}

impl Database {
    
    pub fn new() -> Self{
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&Value>{
        self.entries.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.entries.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<Value>{
        self.entries.remove(key)
    }

}