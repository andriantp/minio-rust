use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use log;

use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use crate::s3::utils;

use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct ObjectInfo {
    pub etag: Option<String>,
    pub size: Option<i64>,
    pub last_modified: Option<String>,
    pub content_type: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Service modular untuk operasi object (file)
pub struct ObjectService<'a> {
    client: &'a Client,
}

impl<'a> ObjectService<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Upload file dari local_path ke key di bucket
    pub async fn upload(
        &self, 
        bucket: &str, 
        local_path: &str, 
        key: &str
    ) -> Result<()> {

        // baca file
        let mut file = File::open(local_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        // hitung SHA256 
        let checksum = utils::sha256_bytes(&buffer);

        // upload dengan metadata: checksum-sha256
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .metadata("checksum-sha256", &checksum)
            .body(ByteStream::from(buffer))
            .send()
            .await?;

        log::info!("[upload] checksum: {}", checksum);
        Ok(())
    }


    /// Download object dari bucket ke local_path
    pub async fn download(&self,bucket : &str, local_path: &str, key: &str) -> Result<()> {
        log::info!(
            "[object.download] downloading '{}' → '{}'",
            key,
            local_path
        );

        let resp = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?.into_bytes();

        let mut file = File::create(local_path).await?;
        file.write_all(&data).await?;

        log::info!("[object.download] download OK");
        Ok(())
    }

    /// List objects berdasarkan prefix
    pub async fn list(&self, bucket : &str,prefix: &str) -> Result<Vec<String>> {
        log::info!("[object.list] prefix: {}", prefix);

        let resp = self.client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .send()
            .await?;

        let objects = resp
            .contents()
            .iter()
            .filter_map(|o| o.key().map(|s| s.to_string()))
            .collect::<Vec<_>>();

        Ok(objects)
    }

    /// Delete object berdasarkan key
    pub async fn delete(&self, bucket : &str, key: &str) -> Result<()> {
        log::warn!("[object.delete] Deleting '{}'", key);

        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        log::info!("[object.delete] Deleted '{}'", key);
        Ok(())
    }

    pub async fn info(&self, bucket: &str, key: &str) -> Result<ObjectInfo> {
        let resp = self.client
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(ObjectInfo {
            etag: resp.e_tag().map(|s| s.to_string()),
            size: resp.content_length(),
            last_modified: resp.last_modified().map(|dt| dt.to_string()),
            content_type: resp.content_type().map(|s| s.to_string()),
            metadata: resp.metadata().cloned().unwrap_or_default(),
        })
    }
  
}
