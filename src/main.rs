use anyhow::Error;
use mini_redis::{config::Config, database::LocolDB, server::Server};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::parse();
    let database = LocolDB::new(&config);
    let server = Server::new(config, database);

    server.run().await?;

    Ok(())
}
