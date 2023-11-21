#![feature(lazy_cell)]
#[macro_use]
extern crate maplit;

use clap::Parser;
use config::Config;

mod config;
mod http;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _cfg = Config::parse();

    http::serve().await?;

    Ok(())
}
