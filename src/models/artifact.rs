use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;


#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Artifact {
    pub sha1: String,
    pub size: u64,
    #[ts(type = "string")]
    pub download_url: Url,
}

impl Artifact {
    pub fn new(sha1: String, size: u64, download_url: Url) -> Self {
        Self {
            sha1,
            size,
            download_url,
        }
    }
}