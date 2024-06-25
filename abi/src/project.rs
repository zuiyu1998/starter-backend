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
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProject {
    pub meta: StarterProjectMeta,
    pub excuter: Executer,
}

impl StarterProject {
    pub fn executer(self) -> Result<()> {
        self.excuter.execute(self.meta)?;

        Ok(())
    }
}

impl StarterProject {
    pub fn new(meta: StarterProjectMeta, excuter: Executer) -> Self {
        Self { meta, excuter }
    }
}

//启动器项目
#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectMeta {
    //
    pub uuid: Uuid,

    pub path: String,
    pub exe_path: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub icon: String,
}

impl StarterProjectMeta {
    pub fn new(exe_path: &str, path: &str, icon: &str) -> Self {
        let now: NaiveDateTime = get_now();

        Self {
            uuid: Uuid::new_v4(),
            path: path.to_string(),
            exe_path: exe_path.to_string(),
            create_at: now.clone(),
            update_at: now,
            icon: icon.to_string(),
        }
    }
}
