pub mod tag;

use abi::tokio::sync::Mutex;
pub use tag::*;

use crate::tag::TagIndexRepo;
use crate::Result;
use abi::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct DocIndex {
    pub tag: Arc<Mutex<dyn TagIndexRepo>>,
}

impl DocIndex {
    pub async fn from_config(config: &Config) -> Result<Self> {
        let tag = TantivyTag::new(config).await?;

        Ok(DocIndex {
            tag: Arc::new(Mutex::new(tag)),
        })
    }
}
