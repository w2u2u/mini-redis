use std::collections::HashMap;

use crate::config::Config;

pub trait Database {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

#[derive(Clone)]
pub struct LocolDB {
    data: HashMap<String, String>,
}

impl LocolDB {
    pub fn new(_config: &Config) -> Self {
        LocolDB {
            data: HashMap::new(),
        }
    }
}

impl Database for LocolDB {
    fn get(&self, key: &str) -> Option<String> {
        println!("->> {:<10} {:#?}", "[Database]", self.data);
        println!("->> {:<10} {:<10} {:<5}", "[Database]", "LocalDB", "Get");
        println!("->> {:<10} {:<10} {key}", "[Database]", "Key");

        if let Some(value) = self.data.get(key).cloned() {
            println!("->> {:<10} {:<10} {value}", "[Database]", "Value");
            Some(value)
        } else {
            println!("->> {:<10} {:<10} not found", "[Database]", "Value");
            None
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        println!("->> {:<10} {:<10} {:<5}", "[Database]", "LocalDB", "Set");
        println!("->> {:<10} {:<10} {key} ", "[Database]", "Key");
        println!("->> {:<10} {:<10} {value}", "[Database]", "Value");

        self.data.insert(key.to_string(), value.to_string());

        println!("->> {:<10} {:#?}", "[Database]", self.data);
    }
}
