use std::sync::Arc;

use anyhow::Error;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    sync::Mutex,
};

use crate::database::Database;

pub struct Handler<D: Database> {
    db: Arc<Mutex<D>>,
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl<D> Handler<D>
where
    D: Database + Clone + Send + Sync + 'static,
{
    pub fn new(db: Arc<Mutex<D>>, tcp_stream: TcpStream) -> Self {
        Handler {
            db,
            stream: BufWriter::new(tcp_stream),
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn process(&mut self) -> Result<(), Error> {
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
                    if let Some(value) = self.db.lock().await.get(key) {
                        value
                    } else {
                        "undefined".to_string()
                    }
                }
                (Some("SET"), Some(key), Some(value)) => {
                    self.db.lock().await.set(key, value);
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
