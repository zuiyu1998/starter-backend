use crate::Result;
use abi::{async_trait::async_trait, prelude::*};

#[async_trait]
pub trait ProjectRepo: 'static + Send + Sync {
    async fn get_project_list(&self) -> Result<StarterProjectListResponse>;
}
