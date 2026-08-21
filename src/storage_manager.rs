use anyhow::Result;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials};

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

        Ok(StorageManager { client: rustfs_client })
    }
}