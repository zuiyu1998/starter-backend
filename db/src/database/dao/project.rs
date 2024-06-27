use crate::database::project::ProjectRepo;
use crate::Result;
use abi::{
    async_trait::async_trait,
    prelude::*,
    sea_orm::{prelude::*, DatabaseConnection, IntoActiveModel},
};

use entity::*;

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
    async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<StarterProjectListResponse> {
        let sql = ProjectEntity::find();

        let count = sql.clone().count(&self.conn).await?;

        let paginate = sql.paginate(&self.conn, params.page_size as u64);

        let data = paginate
            .fetch_page(params.page as u64)
            .await?
            .into_iter()
            .map(|model| StarterProject::from(model))
            .collect::<Vec<StarterProject>>();

        let mut has_next = true;

        if data.len() < params.page as usize {
            has_next = false;
        }

        Ok(StarterProjectListResponse {
            count: count as i32,
            data,
            page: params.page,
            page_size: params.page_size,
            has_next,
        })
    }

    async fn create_project(&self, create: StarterProjectCreate) -> Result<StarterProject> {
        let active = create.into_active_model();

        let model = active.insert(&self.conn).await?;

        Ok(StarterProject::from(model))
    }
}
