use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::{
    executer::{Cmd, CmdPath, Executer, ExecuterKind},
    prelude::ProjectExecuter,
    utils::get_now,
    Result,
};

pub fn from_executer(executer: Executer) -> i32 {
    match executer {
        Executer::Custom => 1,
        Executer::Kind(kind) => match kind {
            ExecuterKind::Cmd(cmd) => match cmd {
                Cmd::Path(_) => 2,
            },
        },
    }
}

pub fn into_executer(num: i32) -> Executer {
    match num {
        1 => Executer::Custom,
        2 => Executer::from(CmdPath),
        _ => Executer::Custom,
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectListResponse {
    pub count: i32,
    pub data: Vec<StarterProject>,
    pub page_size: i32,
    pub page: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProjectCreate {
    pub path: String,
    pub exe_path: String,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub executer: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StarterProject {
    pub meta: StarterProjectMeta,
    pub executer: Executer,
}

impl StarterProject {
    pub fn executer(self) -> Result<()> {
        self.executer.execute(self.meta)?;

        Ok(())
    }
}

impl StarterProject {
    pub fn new(meta: StarterProjectMeta, executer: Executer) -> Self {
        Self { meta, executer }
    }
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
}

impl StarterProjectMeta {
    pub fn new(exe_path: &str, path: &str, icon: &str, name: &str, description: &str) -> Self {
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
        }
    }
}
