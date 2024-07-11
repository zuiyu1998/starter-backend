pub mod error;

pub use abi;

use abi::{config::Config, dashmap::DashMap, prelude::*, uuid::Uuid};
use db::database::Db;
use doc_index::doc_index::DocIndex;
use error::*;

#[derive(Clone)]
pub struct State {
    pub database: Db,
    pub project_map: DashMap<Uuid, StarterProject>,
    pub doc_index: DocIndex,
}

impl State {
    pub async fn update_project(&self, update: StarterProjectUpdate) -> Result<()> {
        let mut is_tags_update = false;

        if update.tags.is_some() {
            is_tags_update = true;
        }

        let project = self.database.project.update_project(update).await?;

        if is_tags_update {
            self.store_tag_index(&project.meta.tags, project.id).await?;
        }

        self.project_map
            .insert(project.meta.uuid.clone(), project.clone());

        Ok(())
    }

    async fn get_ids(&self, tag: &str, page_size: i32, page: i32) -> Result<Vec<i32>> {
        let guard = self.doc_index.tag.lock().await;

        let ids = guard.get_indexs(&tag, page_size, page)?;
        Ok(ids)
    }

    pub async fn get_executer_options() -> Result<Vec<ExecuterOption>> {
        Ok(Executer::all()
            .iter()
            .map(|executer| ExecuterOption {
                executer: executer.as_i32(),
                name: format!("{}", executer),
            })
            .collect())
    }

    pub async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<StarterProjectListResponse> {
        let tags = params.tags.unwrap_or("*".to_string());

        let ids = self.get_ids(&tags, params.page_size, params.page).await?;

        let count = self.database.project.get_count().await?;
        let projects = self.database.project.get_list_by_ids(ids).await?;

        for project in projects.iter() {
            if !self.project_map.contains_key(&project.meta.uuid) {
                self.project_map
                    .insert(project.meta.uuid.clone(), project.clone());
            }
        }

        let has_next = projects.len() >= params.page_size as usize;

        let res = StarterProjectListResponse {
            data: projects,
            page: params.page,
            page_size: params.page_size,
            has_next,
            count,
        };

        Ok(res)
    }

    pub async fn create_project(&self, create: StarterProjectCreate) -> Result<()> {
        let project = self.database.project.create_project(create).await?;

        self.store_tag_index(&project.meta.tags, project.id).await?;

        self.project_map
            .insert(project.meta.uuid.clone(), project.clone());

        Ok(())
    }

    pub async fn delete_project(&self, uuid: Uuid) -> Result<()> {
        if let Some((_, project)) = self.project_map.remove(&uuid) {
            self.database.project.delete(project.id).await?;
        }

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
        let doc_index = DocIndex::from_config(config).await?;

        Ok(Self {
            database,
            project_map: Default::default(),
            doc_index,
        })
    }

    pub async fn store_tag_index(&self, text: &str, id: i32) -> Result<()> {
        let mut guard = self.doc_index.tag.lock().await;

        guard.store_index(text, id)?;

        Ok(())
    }
}
