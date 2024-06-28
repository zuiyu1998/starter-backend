pub mod error;

pub use abi;

use abi::{config::Config, dashmap::DashMap, prelude::*, uuid::Uuid};
use db::database::Db;
use error::*;

#[derive(Clone)]
pub struct State {
    pub database: Db,
    pub project_map: DashMap<Uuid, StarterProject>,
}

impl State {
    pub async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<StarterProjectListResponse> {
        let res = self.database.project.get_project_list(params).await?;

        for project in res.data.iter() {
            if !self.project_map.contains_key(&project.meta.uuid) {
                self.project_map
                    .insert(project.meta.uuid.clone(), project.clone());
            }
        }

        Ok(res)
    }

    pub async fn create_project(&self, create: StarterProjectCreate) -> Result<()> {
        let project = self.database.project.create_project(create).await?;

        self.project_map
            .insert(project.meta.uuid.clone(), project.clone());

        Ok(())
    }

    pub async fn delete_project(&self, uuid: Uuid) -> Result<()> {
        self.database.project.delete(uuid).await?;

        self.project_map.remove(&uuid);

        Ok(())
    }

    pub async fn excute_project(&self, uuid: &Uuid) -> Result<()> {
        if let Some(project) = self.project_map.get(uuid) {
            let project = project.clone();

            project.executer()?;
        }

        Ok(())
    }

    pub async fn from_config(config: &Config) -> Result<Self> {
        let database = Db::from_config(config).await?;

        Ok(Self {
            database,
            project_map: Default::default(),
        })
    }
}
