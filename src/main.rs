use anyhow::Result;

pub mod bundle;
pub mod cli;
pub mod compiler;
pub mod config;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
