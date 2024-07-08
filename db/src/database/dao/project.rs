use crate::database::project::ProjectRepo;
use crate::Result;
use abi::{
    async_trait::async_trait,
    prelude::*,
    sea_orm::{prelude::*, DatabaseConnection, IntoActiveModel, Set},
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
        let sql = ProjectEntity::find().filter(ProjectColumn::IsEnable.eq(true));

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

    async fn delete(&self, id: i32) -> Result<StarterProject> {
        let mut active: ProjectActiveModel = Default::default();

        active.id = Set(id);
        active.is_delete = Set(true);
        active.is_enable = Set(false);

        let model = active.update(&self.conn).await?;

        Ok(StarterProject::from(model))
    }

    async fn get_list_by_ids(&self, ids: Vec<i32>) -> Result<Vec<StarterProject>> {
        let sql = ProjectEntity::find().filter(ProjectColumn::Id.is_in(ids));

        let data = sql
            .all(&self.conn)
            .await?
            .into_iter()
            .map(|model| StarterProject::from(model))
            .collect();

        Ok(data)
    }

    async fn get_count(&self) -> Result<i32> {
        let count = ProjectEntity::find().count(&self.conn).await?;

        Ok(count as i32)
    }

    async fn update_project(&self, update: StarterProjectUpdate) -> Result<StarterProject> {
        let active = update.into_active_model();

        let model = active.update(&self.conn).await?;

        Ok(StarterProject::from(model))
    }
}
