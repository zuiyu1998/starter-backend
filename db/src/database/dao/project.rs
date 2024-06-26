use crate::database::project::ProjectRepo;
use crate::Result;
use abi::{
    async_trait::async_trait,
    prelude::*,
    sea_orm::{prelude::*, DatabaseConnection, IntoActiveModel},
};

pub struct DaoPoject {
    conn: DatabaseConnection,
}

impl DaoPoject {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl ProjectRepo for DaoPoject {
    async fn get_project_list(&self) -> Result<StarterProjectListResponse> {
        Ok(StarterProjectListResponse {
            count: 1,
            data: vec![StarterProject::new(
                StarterProjectMeta::new("code", ".", "", "", ""),
                Executer::from(CmdPath),
            )],
            page: 1,
            page_size: 50,
        })
    }

    async fn create_project(&self, create: StarterProjectCreate) -> Result<StarterProject> {
        let active = create.into_active_model();

        let model = active.insert(&self.conn).await?;

        Ok(StarterProject::from(model))
    }
}
