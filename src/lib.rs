use std::sync::Arc;

use anyhow::Result;
use tonic::transport::Server;

use crate::storage_service::{StorageService, storage::storage_server::StorageServer};

pub mod config;
mod storage_manager;
mod storage_service;

pub async fn run(conf: &config::Config) -> Result<()> {
    let addr = format!("{}:{}", &conf.service.host, &conf.service.port).parse()?;
    let storage_manager = storage_manager::StorageManager::build(&conf.rustfs)
        .await?;
    let storage_manager_arc = Arc::new(storage_manager);
    let storage_service = StorageService::build(Arc::clone(&storage_manager_arc));

    println!("storage service started on port {}", &conf.service.port);
    Server::builder()
        .add_service(StorageServer::new(storage_service))
        .serve(addr)
        .await?;
    println!("storage service stopped");
    Ok(())
}