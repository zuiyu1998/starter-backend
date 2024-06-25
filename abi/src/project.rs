use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{executer::Executer, utils::get_now};

pub struct StarterProject {
    pub meta: StarterProjectMeta,
    pub excuter: Executer,
}

impl StarterProject {
    pub fn new(meta: StarterProjectMeta, excuter: Executer) -> Self {
        Self { meta, excuter }
    }
}

//启动器项目
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
