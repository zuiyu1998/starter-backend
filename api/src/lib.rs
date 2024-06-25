pub mod error;

pub use abi;

use abi::{dashmap::DashMap, prelude::*, uuid::Uuid};
use db::database::Database;
use error::*;

#[derive(Clone)]
pub struct State {
    pub database: Database,
    pub project_map: DashMap<Uuid, StarterProject>,
}

impl State {
    pub async fn get_project_list(&self) -> Result<StarterProjectListResponse> {
        let res = self.database.project.get_project_list().await?;

        for project in res.data.iter() {
            if !self.project_map.contains_key(&project.meta.uuid) {
                self.project_map
                    .insert(project.meta.uuid.clone(), project.clone());
            }
        }

        Ok(res)
    }

    pub async fn excute_project(&self, uuid: &Uuid) -> Result<()> {
        if let Some(project) = self.project_map.get(uuid) {
            let project = project.clone();

            project.executer()?;
        }

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            database: Database::new(),
            project_map: Default::default(),
        }
    }
}
