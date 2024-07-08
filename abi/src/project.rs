use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::{executer::Executer, prelude::ProjectExecuter, utils::get_now, Result};

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectListResponse {
    pub count: i32,
    pub data: Vec<StarterProject>,
    pub page_size: i32,
    pub page: i32,
    pub has_next: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectCreate {
    pub path: String,
    pub exe_path: String,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub executer: i32,
    pub tags: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectUpdate {
    pub id: i32,
    pub path: Option<String>,
    pub exe_path: Option<String>,
    pub icon: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub executer: Option<i32>,
    pub tags: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProject {
    pub meta: StarterProjectMeta,
    pub executer: i32,
    pub id: i32,
}

impl StarterProject {
    pub fn executer(self) -> Result<()> {
        let executer = Executer::from(self.executer);

        executer.execute(self.meta)?;

        Ok(())
    }
}

impl StarterProject {
    pub fn new(meta: StarterProjectMeta, executer: Executer, id: i32) -> Self {
        Self {
            meta,
            executer: executer.as_i32(),
            id,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GetProjectListParams {
    pub page_size: i32,
    pub page: i32,
    pub tags: Option<String>,
}

//启动器项目
#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectMeta {
    pub uuid: Uuid,
    pub path: String,
    pub exe_path: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub tags: String,
}

impl StarterProjectMeta {
    pub fn new(
        exe_path: &str,
        path: &str,
        icon: &str,
        name: &str,
        description: &str,
        tags: &str,
    ) -> Self {
        let now: NaiveDateTime = get_now();

        Self {
            uuid: Uuid::new_v4(),
            path: path.to_string(),
            exe_path: exe_path.to_string(),
            create_at: now.clone(),
            update_at: now,
            icon: icon.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            tags: tags.to_string(),
        }
    }
}
