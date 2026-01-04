use anyhow::Result;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::{
    BucketLocationConstraint,
    CreateBucketConfiguration,
};
use log;

/// Statistik bucket sederhana
pub struct BucketStats {
    pub object_count: usize,
    pub total_size: u64,
}

/// Service modular untuk operasi bucket
pub struct BucketService<'a> {
    client: &'a Client,
}

impl<'a> BucketService<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List semua bucket
    pub async fn list(&self) -> Result<Vec<String>> {
        let resp = self.client.list_buckets().send().await?;

        let names = resp
            .buckets()
            .iter()
            .filter_map(|b| b.name().map(|s| s.to_string()))
            .collect();

        Ok(names)
    }

    /// Cek apakah bucket ada
    pub async fn exists(&self, bucket: &str) -> Result<bool> {
        let resp = self.client.list_buckets().send().await?;

        let exists = resp
            .buckets()
            .iter()
            .any(|b| b.name().unwrap_or_default() == bucket);

        Ok(exists)
    }

    /// Create bucket baru
    pub async fn create(&self, bucket: &str, region: &str) -> Result<()> {
        log::info!("[bucket.create] Creating bucket: {}", bucket);

        self.client
            .create_bucket()
            .bucket(bucket)
            .create_bucket_configuration(
                CreateBucketConfiguration::builder()
                    .location_constraint(
                        BucketLocationConstraint::from(region)
                    )
                    .build()
            )
            .send()
            .await?;

        log::info!("[bucket.create] Bucket '{}' created", bucket);

        Ok(())
    }

    /// Pastikan bucket ada — jika tidak, buat baru
    pub async fn ensure(&self, bucket: &str, region: &str) -> Result<()> {
        if self.exists(bucket).await? {
            log::info!("[bucket.ensure] Bucket '{}' exists", bucket);
            return Ok(());
        }

        log::warn!("[bucket.ensure] Bucket '{}' not found — creating", bucket);
        self.create(bucket, region).await?;
        Ok(())
    }

    /// Delete bucket (HARUS kosong)
    pub async fn delete(&self, bucket: &str) -> Result<()> {
        log::warn!("[bucket.delete] Deleting bucket: {}", bucket);

        self.client
            .delete_bucket()
            .bucket(bucket)
            .send()
            .await?;

        log::info!("[bucket.delete] Bucket '{}' deleted", bucket);

        Ok(())
    }

    /// Hapus semua object dalam bucket
    pub async fn delete_objects(&self, bucket: &str) -> Result<()> {
        log::warn!("[bucket.delete_objects] Deleting ALL objects in {}", bucket);

        // Ambil daftar object
        let resp = self.client
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await?;

        if resp.contents().is_empty() {
            log::info!("[bucket.delete_objects] Bucket already empty");
            return Ok(());
        }

        // Buat batch delete entries
        let objects = resp
            .contents()
            .iter()
            .filter_map(|o| o.key().map(|k| k.to_string()))
            .collect::<Vec<_>>();

        for key in objects {
            self.client
                .delete_object()
                .bucket(bucket)
                .key(&key)
                .send()
                .await?;

            log::info!("[bucket.delete_objects] Deleted: {}", key);
        }

        Ok(())
    }

    /// Statistik bucket: total object & total size
    pub async fn stats(&self, bucket: &str) -> Result<BucketStats> {
        let resp = self.client
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await?;

        let mut count = 0;
        let mut size = 0;

        for obj in resp.contents() {
            count += 1;
            size += obj.size().map(|s| s as u64).unwrap_or(0);
        }

        Ok(BucketStats {
            object_count: count,
            total_size: size,
        })
    }
}

