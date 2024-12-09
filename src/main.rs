mod cli;
mod db;
mod jwt;
mod server;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    cli::cli().await.unwrap();
}
