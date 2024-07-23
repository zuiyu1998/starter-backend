use crate::Result;
use abi::{async_trait::async_trait, prelude::*};

#[async_trait]
pub trait ProjectRepo: 'static + Send + Sync {
    async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<StarterProjectListResponse>;
    async fn create_project(&self, create: StarterProjectCreate) -> Result<StarterProject>;

    async fn delete(&self, id: i32) -> Result<StarterProject>;

    async fn get_list_by_ids(&self, ids: Vec<i32>) -> Result<Vec<StarterProject>>;

    async fn get_count(&self) -> Result<i32>;

    async fn update_project(&self, update: StarterProjectUpdate) -> Result<StarterProject>;
}
