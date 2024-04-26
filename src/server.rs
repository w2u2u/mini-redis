use std::sync::Arc;

use anyhow::Error;
use tokio::{net::TcpListener, sync::Mutex};

use crate::{config::Config, database::Database, handler::Handler};

pub struct Server<D: Database> {
    config: Config,
    db: D,
}

impl<D> Server<D>
where
    D: Database + Clone + Send + Sync + 'static,
{
    pub fn new(config: Config, db: D) -> Self {
        Server { config, db }
    }

    pub async fn run(&self) -> Result<(), Error> {
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
