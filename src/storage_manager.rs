use anyhow::Result;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials, primitives::ByteStream};
use uuid::Uuid;
use crate::config;

pub struct StorageManager {
    client: Client,
}

impl StorageManager {
    pub async fn build(conf: &config::RustfsConfig) -> Result<StorageManager> {
        let credentials = Credentials::new(
            &conf.access_key,
            &conf.secret_key,
            None,
            None,
            "rustfs",
        );

        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(conf.region.clone()))
            .credentials_provider(credentials)
            .endpoint_url(&conf.endpoint_url)
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();

        let rustfs_client = Client::from_conf(s3_config);
        let buckets = rustfs_client.list_buckets().send().await?;
        let music_bucket = buckets.buckets().iter()
            .find(|&b| b.name == Some("music-bucket".to_string()));
        match music_bucket {
            Some(_) => (),
            None => {
                rustfs_client.create_bucket()
                    .bucket("music-bucket")
                    .send()
                    .await?;
            },
        };
        Ok(StorageManager { client: rustfs_client })
    }

    pub async fn upload_audio(&self, file: &[u8]) -> Result<String> {
        let key = Uuid::new_v4().to_string();
        self.client
            .put_object()
            .bucket("music-bucket")
            .key(&key)
            .body(ByteStream::from(file.to_vec()))
            .send()
            .await?;
        Ok(key)
    }
}