use async_trait::async_trait;
use std::{collections::HashMap, error, fmt, path::Path};

#[derive(Debug)]
pub struct SSSError(pub String);

impl fmt::Display for SSSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for SSSError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

/// A trait encapsulating the operations required of LayerKV.
#[async_trait]
pub trait SSSFacade: Sync + Send {
    async fn init() -> Self;
    async fn upload(
        &self,
        key: &str,
        file_path: &Path,
        meta: Option<HashMap<&str, &str>>,
    ) -> Result<bool, SSSError>;
    async fn delete(&self, key: &str) -> Result<bool, SSSError>;
    async fn download(&self, key: &str, save_path: &Path) -> Result<bool, SSSError>;
    async fn head<'a>(&self, key: &str) -> Result<HashMap<&'a str, &'a str>, SSSError>;
}
