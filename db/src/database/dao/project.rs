use crate::database::project::ProjectRepo;
use crate::Result;
use abi::{async_trait::async_trait, prelude::*};

pub struct DaoPoject;

#[async_trait]
impl ProjectRepo for DaoPoject {
    async fn get_project_list(&self) -> Result<StarterProjectListResponse> {
        Ok(StarterProjectListResponse {
            count: 1,
            data: vec![StarterProject::new(
                StarterProjectMeta::new("code", ".", ""),
                Executer::from(CmdPath),
            )],
            page: 1,
            page_size: 50,
        })
    }
}
