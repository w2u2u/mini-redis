use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use bytes::BytesMut;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{TcpListener, TcpStream},
};

struct Config {
    host: String,
    port: String,
}

impl Config {
    fn parse() -> Self {
        let _args: Vec<String> = env::args().collect();

        Config {
            host: "127.0.0.1".to_string(),
            port: "8080".to_string(),
        }
    }
}

trait Database {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

#[derive(Clone)]
struct LocolDB {
    data: HashMap<String, String>,
}

impl LocolDB {
    fn new(_config: &Config) -> Self {
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

struct Handler<D: Database> {
    db: Arc<Mutex<D>>,
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl<D> Handler<D>
where
    D: Database + Clone + Send + Sync + 'static,
{
    fn new(db: Arc<Mutex<D>>, tcp_stream: TcpStream) -> Self {
        Handler {
            db,
            stream: BufWriter::new(tcp_stream),
            buffer: BytesMut::with_capacity(1024),
        }
    }

    async fn process(&mut self) -> Result<(), Error> {
        self.stream.read_buf(&mut self.buffer).await?;
        println!(
            "->> {:<10} {:<10} {:#?}",
            "[Handler]", "Received", self.buffer
        );

        if let Ok(buffer) = String::from_utf8(self.buffer.to_vec()) {
            let buffers: Vec<&str> = buffer.split_whitespace().collect();

            let res = match (
                buffers.first().copied(),
                buffers.get(1).copied(),
                buffers.get(2).copied(),
            ) {
                (Some("GET"), Some(key), None) => {
                    if let Some(value) = self.db.lock().unwrap().get(key) {
                        value
                    } else {
                        "undefined".to_string()
                    }
                }
                (Some("SET"), Some(key), Some(value)) => {
                    self.db.lock().unwrap().set(key, value);
                    "OK".to_string()
                }
                _ => "Invalid command".to_string(),
            };

            println!("->> {:<10} {:<10} {res}", "[Handler]", "Response");

            self.stream.write_all(res.as_bytes()).await?;
            self.stream.flush().await?;
        }

        Ok(())
    }
}

struct Server<D: Database> {
    config: Config,
    db: D,
}

impl<D> Server<D>
where
    D: Database + Clone + Send + Sync + 'static,
{
    fn new(config: Config, db: D) -> Self {
        Server { config, db }
    }

    async fn run(&self) -> Result<(), Error> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(addr).await?;

        println!(
            "->> {:<10} Running on {}:{}",
            "[Server]", self.config.host, self.config.port
        );

        let database = Arc::new(Mutex::new(self.db.clone()));

        while let Ok((stream, _)) = listener.accept().await {
            println!("->> {}", "=".repeat(50));
            println!("->> {:<10} {:<10} TCP Connection", "[Server]", "Accept");

            let database = Arc::clone(&database);
            let mut handler = Handler::new(database, stream);

            tokio::spawn(async move {
                handler.process().await.unwrap();
            });
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::parse();
    let database = LocolDB::new(&config);
    let server = Server::new(config, database);

    server.run().await?;

    Ok(())
}
