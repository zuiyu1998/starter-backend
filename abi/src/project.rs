use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::utils::get_now;

//启动器项目
pub struct StarterProject {
    //
    pub uuid: Uuid,

    pub path: PathBuf,
    pub exe_path: PathBuf,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub icon: String,
}

impl StarterProject {
    pub fn new(path: &Path, exe_path: &Path, icon: &str) -> Self {
        let now: NaiveDateTime = get_now();

        Self {
            uuid: Uuid::new_v4(),
            path: path.to_path_buf(),
            exe_path: exe_path.to_path_buf(),
            create_at: now.clone(),
            update_at: now,
            icon: icon.to_string(),
        }
    }
}
