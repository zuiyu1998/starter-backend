use directories::ProjectDirs;
use std::path::PathBuf;

const APP_NAME: &'static str = "stater";
const APP_ORGANIZATION: &'static str = "next";

fn get_config_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", APP_ORGANIZATION, APP_NAME).unwrap();
    proj_dirs.config_dir().to_path_buf()
}

#[derive(Default)]
pub struct Config {
    pub system: SystemConfig,
}

pub struct SystemConfig {
    pub config: PathBuf,
}

impl Default for SystemConfig {
    fn default() -> Self {
        SystemConfig {
            config: get_config_dir(),
        }
    }
}

impl SystemConfig {
    pub fn get_db_dir(&self) -> PathBuf {
        self.config.join("db")
    }

    pub fn get_sqlite_path(&self) -> PathBuf {
        let mut path = self.get_db_dir();
        path.push("sqlite.db");

        path
    }
}
