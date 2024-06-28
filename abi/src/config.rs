use crate::Result;
use directories::ProjectDirs;
use std::path::PathBuf;

use figment::{
    providers::{Format, Serialized, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

const APP_NAME: &'static str = "stater";
const APP_ORGANIZATION: &'static str = "next";

fn get_config_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", APP_ORGANIZATION, APP_NAME).unwrap();
    proj_dirs.config_dir().to_path_buf()
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub system: SystemConfig,
}

impl Config {
    pub const DEFAULT_YAML: &'static str = "./config.yaml";
    pub const DEFAULT_TEST_YAML: &'static str = "./config_test.yaml";

    pub fn load() -> Result<Self> {
        #[cfg(not(debug_assertions))]
        let config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(Self::DEFAULT_YAML))
            .extract()?;

        #[cfg(debug_assertions)]
        let config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(Self::DEFAULT_TEST_YAML))
            .extract()?;

        Ok(config)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SystemConfig {
    pub config: PathBuf,
    pub sqlite_db_name: String,
}

impl Default for SystemConfig {
    fn default() -> Self {
        SystemConfig {
            config: get_config_dir(),
            sqlite_db_name: "sqlite.db".to_string(),
        }
    }
}

impl SystemConfig {
    pub fn get_db_dir(&self) -> PathBuf {
        self.config.join("db")
    }

    pub fn get_sqlite_path(&self) -> PathBuf {
        let mut path = self.get_db_dir();
        path.push(&self.sqlite_db_name);

        path
    }
}
