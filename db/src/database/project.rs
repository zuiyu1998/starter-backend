use crate::Result;
use abi::{async_trait::async_trait, prelude::*, uuid::Uuid};

#[async_trait]
pub trait ProjectRepo: 'static + Send + Sync {
    async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<StarterProjectListResponse>;
    async fn create_project(&self, create: StarterProjectCreate) -> Result<StarterProject>;

    async fn delete(&self, uuid: Uuid) -> Result<StarterProject>;
}
