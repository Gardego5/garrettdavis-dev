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
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
    let ses = aws_sdk_sesv2::Client::new(&config);
    let cfg = Config::parse();
    let ctx = http::context::Ctx { ses, cfg };

    http::serve(ctx).await?;

    Ok(())
}
