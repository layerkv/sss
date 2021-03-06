use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, error, fmt, path::Path};

pub type SSSMeta = HashMap<String, String>;

/// A trait encapsulating the operations required of LayerKV.
#[async_trait]
pub trait SSSFacade: Sync + Send + Sized {
    async fn init(access: &SSSAccess, region: &str, bucket: &str) -> Result<Self, SSSErr>;
    async fn upload(
        &self,
        key: &str,
        file_path: &Path,
        meta: Option<SSSMeta>,
    ) -> Result<bool, SSSErr>;
    async fn delete(&self, key: &str) -> Result<bool, SSSErr>;
    async fn download(&self, key: &str, save_path: &Path) -> Result<bool, SSSErr>;
    async fn head(&self, key: &str) -> Result<SSSHead, SSSErr>;
}

#[derive(Debug)]
pub struct SSSAccess {
    pub access_id: String,
    pub access_secret: String,
}

#[derive(Debug)]
pub struct SSSHead {
    pub content_length: i64,
    pub e_tag: String,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug)]
pub struct SSSErr(pub String);

impl fmt::Display for SSSErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for SSSErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
