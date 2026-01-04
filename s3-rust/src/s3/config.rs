use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct S3Config {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub endpoint: String,
    pub bucket: String,
}

impl S3Config {
    pub fn new() -> Self {
        // Load from .env (dotenvy already called in main or client)
        let cfg = Self {
            access_key: env::var("AWS_ACCESS_KEY_ID")
                .unwrap_or_else(|_| "minioadmin".to_string()),

            secret_key: env::var("AWS_SECRET_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),

            region: env::var("AWS_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),

            endpoint: env::var("MINIO_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),

            bucket: env::var("MINIO_BUCKET")
                .unwrap_or_else(|_| "mybucket".to_string()),
        };

        log::info!("🔧 Loaded S3 config: {:?}", cfg);
        cfg
    }
}
