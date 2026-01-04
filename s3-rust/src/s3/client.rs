use anyhow::Result;

use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::{Client, Config};
use aws_sdk_s3::config::BehaviorVersion;

use crate::s3::config::S3Config;
use crate::s3::bucket::BucketService;
use crate::s3::object::ObjectService;

/// =======================================================================
/// 🎯 REPOSITORY BUILDER
/// - Menyimpan config
/// - Build AWS/MinIO Client
/// - Ensure bucket menggunakan BucketService
/// - Menghasilkan Repository yang sudah valid
/// =======================================================================
pub struct RepositoryBuilder {
    conf: S3Config,
}

impl RepositoryBuilder {
    /// Create new builder from config
    pub fn new(conf: S3Config) -> Self {
        Self { conf }
    }

    /// Build full Repository (client + ensure bucket)
    pub async fn connect(self) -> Result<Repository> {
        log::info!("[builder] Connecting to MinIO at {}", self.conf.endpoint);

        // 1. Setup region
        let region = Region::new(self.conf.region.clone());
        let region_provider = RegionProviderChain::first_try(region.clone())
            .or_else("us-east-1");

        // 2. Load default AWS config
        let base = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        // 3. Build custom S3 config for MinIO
        let config = Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(base.region().unwrap().clone())
            .credentials_provider(base.credentials_provider().unwrap())
            .endpoint_url(self.conf.endpoint.clone())
            .force_path_style(true)
            .build();

        let client = Client::from_conf(config);

        log::info!("[builder] Client initialized OK");

        // 4. Ensure bucket using BucketService
        // let bucket_service = BucketService::new(&client);
        // bucket_service
        //     .ensure(&self.conf.bucket, &self.conf.region)
        //     .await?;

        // log::info!(
        //     "[builder] Bucket '{}' ensured OK",
        //     self.conf.bucket
        // );

        // 5. Return Repository in valid state
        Ok(Repository {
            client,
            conf: self.conf,
        })
    }
}

/// =======================================================================
/// 🎯 REPOSITORY
/// - Berisi client yang valid
/// - Menjadi façade ke bucket/object services
/// - Semua operasi siap dipanggil tanpa state ambiguity
/// =======================================================================
#[derive(Clone)]
pub struct Repository {
    pub client: Client,
    pub conf: S3Config,  
}

impl Repository {
    /// Entry point ke BucketService
    pub fn bucket(&self) -> BucketService<'_> {
        BucketService::new(&self.client)
    }

    pub fn object(&self) -> ObjectService<'_> {
        ObjectService::new(&self.client)
    }



}
