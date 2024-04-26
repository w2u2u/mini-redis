use std::env;

pub struct Config {
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn parse() -> Self {
        let _args: Vec<String> = env::args().collect();

        Config {
            host: "127.0.0.1".to_string(),
            port: "8080".to_string(),
        }
    }
}
