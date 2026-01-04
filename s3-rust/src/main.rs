use dotenvy;
use env_logger::Env;
use anyhow::Result;

mod s3;
use s3::config::S3Config;
use s3::client::RepositoryBuilder;
use crate::s3::utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Mode {
    // bucket mode
    BucketList,
    BucketExists {
        name: String,
    },
    BucketCreate {
        name: String,
    },
    BucketDelete {
        name: String,
    },
    BucketStats {
        name: String,
    },

    // object mode
    ObjectList {
        bucket: String,
        prefix: String,
    },
    ObjectInfo {
        bucket: String,
        key: String,
    },
    ObjectUpload {
        bucket: String,
        path: String,
        key: String,
    },
    ObjectDownload {
        bucket: String,
        path: String,
        key: String,
    },
    ObjectDelete {
        bucket: String,
        key: String,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load .env
    if dotenvy::dotenv().is_err() {
        panic!("⚠️ .env file not found");
    }

    log::info!("🚀 Application started");

    // 1) Load config
    let conf = S3Config::new();

    // 2) Connect via builder
    let repo = RepositoryBuilder::new(conf)
        .connect()
        .await?;

    // 3) Parse CLI
    let cli = Cli::parse();

    // 4) Route
    match cli.mode {
        // bucket
        Mode::BucketList => bucket_list(&repo).await?,
        Mode::BucketCreate { name } => bucket_create(&repo, &name, &repo.conf.region).await?,
        Mode::BucketExists { name } => bucket_exists(&repo, &name).await?,
        Mode::BucketDelete { name } => bucket_delete(&repo, &name).await?,
        Mode::BucketStats { name } => bucket_stats(&repo, &name).await?,

        // object
        Mode::ObjectUpload{ bucket  , path, key} => object_upload(&repo, &bucket  , &path, &key).await?,
        Mode::ObjectList { bucket  , prefix} => object_list(&repo, &bucket  , &prefix).await?,
        Mode::ObjectInfo{ bucket  , key} => object_info(&repo, &bucket  ,  &key).await?,
        Mode::ObjectDownload{ bucket  , path, key} => object_download(&repo, &bucket  , &path, &key).await?,
        Mode::ObjectDelete{ bucket  ,  key} => object_delete(&repo, &bucket  ,  &key).await?,  
    }

    Ok(())
}

/// ========= Bucket handlers =========
async fn bucket_list(repo: &crate::s3::client::Repository) -> Result<()> {
    let list = repo.bucket().list().await?;
    log::info!("Buckets: {:?}", list);
    Ok(())
}

async fn bucket_exists(repo: &crate::s3::client::Repository, bucket: &str) -> Result<()> {
    let exist = repo.bucket().exists(bucket).await?;
    log::info!("Exists?: {:?}", exist);
    Ok(())
}

async fn bucket_create(repo: &crate::s3::client::Repository, bucket: &str, region: &str) -> Result<()> {
    repo.bucket().ensure(bucket, region).await?;
    Ok(())
}

async fn bucket_delete(repo: &crate::s3::client::Repository, bucket: &str) -> Result<()> {
    log::info!("Deleting bucket: {}", bucket);

    // Optional: you can delete objects first
    repo.bucket().delete_objects(bucket).await?;

    repo.bucket().delete(bucket).await?;
    log::info!("Bucket deleted");
    Ok(())
}

async fn bucket_stats(repo: &crate::s3::client::Repository, bucket: &str) -> Result<()> {
    let stats = repo.bucket().stats(bucket).await?;

    log::info!(
        "Bucket '{}' — objects: {}, size: {} bytes",
        bucket,
        stats.object_count,
        stats.total_size
    );

    Ok(())
}

/// ========= Object handlers =========
async fn object_list(repo: &crate::s3::client::Repository, bucket: &str, prefix: &str) -> Result<()> {
    let list = repo.object().list(bucket, prefix).await?;
    log::info!("Object: {:?}", list);
    Ok(())
}

async fn object_info(repo: &crate::s3::client::Repository, bucket: &str,  key: &str) -> Result<()> {
    let info= repo.object().info(bucket, key).await?;
    let json = serde_json::to_string_pretty(&info)?;
    log::info!("Info:\n{}", json);
    Ok(())
}

async fn object_upload(repo: &crate::s3::client::Repository, bucket: &str, path: &str, key: &str) -> Result<()> {
    repo.object().upload(bucket, path, key ).await?;
    log::info!(
        "Upload Bucket '{}' — path: {}, key: {} succes",
        bucket,
        path,
        key
    );
    Ok(())
}

async fn object_download(repo: &crate::s3::client::Repository, bucket: &str, path: &str, key: &str) -> Result<()> {
    repo.object().download(bucket, path, key ).await?;
    log::info!(
        "Download Bucket '{}' — path: {}, key: {} succes",
        bucket,
        path,
        key
    );

    let info = utils::sha256_file(path).await?;
    log::info!("Downloaded file checksum: {}", info);

    Ok(())
}

async fn object_delete(repo: &crate::s3::client::Repository, bucket: &str,  key: &str) -> Result<()> {
    repo.object().delete(bucket,  key ).await?;
    log::info!(
        "Delete Bucket '{}' — key: {} succes",
        bucket,
        key
    );
    Ok(())
}