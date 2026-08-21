use anyhow::Result;

pub mod config;
mod storage_manager;

pub async fn run(conf: &config::Config) -> Result<()> {
    println!("config:\n{:?}", conf);
    Ok(())
}