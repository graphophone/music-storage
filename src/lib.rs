use anyhow::Result;

pub mod config;

pub async fn run(conf: &config::Config) -> Result<()> {
    println!("config:\n{:?}", conf);
    Ok(())
}