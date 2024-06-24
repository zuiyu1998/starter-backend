use std::path::PathBuf;

use chrono::NaiveDateTime;
use uuid::Uuid;

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

pub trait ProjectExecuter {
    fn executer(&mut self, project: StarterProject);
}
