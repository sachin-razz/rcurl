use anyhow::Result;

/// Multi-Cloud Object Storage Sync Engine (`s3://`, `gcs://`, `azure://`, `b2://`)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudProvider {
    AwsS3,
    GoogleCloudStorage,
    AzureBlob,
    BackblazeB2,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiCloudEngine {
    pub provider: CloudProvider,
    pub bucket: String,
    pub key: String,
}

#[allow(dead_code)]
impl MultiCloudEngine {
    pub fn parse_cloud_uri(uri: &str) -> Result<Self> {
        if let Some(stripped) = uri.strip_prefix("s3://") {
            let (bucket, key) = stripped.split_once('/').unwrap_or((stripped, ""));
            Ok(Self {
                provider: CloudProvider::AwsS3,
                bucket: bucket.to_string(),
                key: key.to_string(),
            })
        } else if let Some(stripped) = uri.strip_prefix("gcs://") {
            let (bucket, key) = stripped.split_once('/').unwrap_or((stripped, ""));
            Ok(Self {
                provider: CloudProvider::GoogleCloudStorage,
                bucket: bucket.to_string(),
                key: key.to_string(),
            })
        } else if let Some(stripped) = uri.strip_prefix("azure://") {
            let (container, key) = stripped.split_once('/').unwrap_or((stripped, ""));
            Ok(Self {
                provider: CloudProvider::AzureBlob,
                bucket: container.to_string(),
                key: key.to_string(),
            })
        } else if let Some(stripped) = uri.strip_prefix("b2://") {
            let (bucket, key) = stripped.split_once('/').unwrap_or((stripped, ""));
            Ok(Self {
                provider: CloudProvider::BackblazeB2,
                bucket: bucket.to_string(),
                key: key.to_string(),
            })
        } else {
            anyhow::bail!("Unsupported cloud URI scheme");
        }
    }

    pub fn build_http_endpoint(&self) -> String {
        match self.provider {
            CloudProvider::AwsS3 => format!("https://{}.s3.amazonaws.com/{}", self.bucket, self.key),
            CloudProvider::GoogleCloudStorage => format!("https://storage.googleapis.com/{}/{}", self.bucket, self.key),
            CloudProvider::AzureBlob => format!("https://{}.blob.core.windows.net/{}", self.bucket, self.key),
            CloudProvider::BackblazeB2 => format!("https://f000.backblazeb2.com/file/{}/{}", self.bucket, self.key),
        }
    }
}
