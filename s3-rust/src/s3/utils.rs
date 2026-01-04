use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use sha2::{Sha256, Digest};

/// Hitung SHA256 dari bytes
pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hitung SHA256 dari file di local path
pub async fn sha256_file(path: &str) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    Ok(sha256_bytes(&buffer))
}
