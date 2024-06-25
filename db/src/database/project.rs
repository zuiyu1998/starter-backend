use crate::Result;
use abi::{async_trait::async_trait, prelude::*};

#[async_trait]
pub trait ProjectRepo {
    async fn get_project_list(&self) -> Result<StarterProjectListResponse>;
}
