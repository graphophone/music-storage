use anyhow::Result;
use tokio::fs;

pub mod config;
mod storage_manager;

pub async fn run(conf: &config::Config) -> Result<()> {
    println!("config:\n{:?}", conf);

    let sm = storage_manager::StorageManager::build(&conf.rustfs)
        .await?;
    println!("connected to rustfs!");

    let file = fs::read("Cargo.toml").await?;
    sm.upload_audio(&file[..]).await?;
    println!("uploaded a file");

    Ok(())
}