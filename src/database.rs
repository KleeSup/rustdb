use std::collections::HashMap;

#[derive(Debug)]
pub enum Value{
    String(String),
    Integer(i64),
    Float(f64),
    List(Vec<String>),
}

#[derive(Default)]
pub struct Database{
    entries: HashMap<String, Value>
}

impl Database {
    
    fn new() -> Self{
        Self { entries: HashMap::new() }
    }

}