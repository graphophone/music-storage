use std::sync::Arc;

use tonic::{Request, Response, Status, async_trait};

use crate::{storage_manager::StorageManager, storage_service::storage::{UploadAudioRequest, UploadAudioResponse, storage_server::Storage}};

pub mod storage {
    tonic::include_proto!("storage");
}

pub struct StorageService {
    storage_manager: Arc<StorageManager>,
}

impl StorageService {
    pub fn build(storage_manager: Arc<StorageManager>) -> StorageService {
        StorageService { storage_manager }        
    }
}

#[async_trait]
impl Storage for StorageService {
    async fn upload_audio(&self, req: Request<UploadAudioRequest>) -> Result<Response<UploadAudioResponse>, Status> {
        let req = req.into_inner();
        let key = self.storage_manager.upload_audio(&req.audio).await;
        let key = match key {
            Ok(v) => v,
            Err(e) => return Err(Status::from_error(e.into_boxed_dyn_error())),
        };
        Ok(Response::new(UploadAudioResponse { key }))
    }
}