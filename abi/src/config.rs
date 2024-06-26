use std::path::PathBuf;

pub struct Config {
    pub system: SystemConfig,
}

pub struct SystemConfig {
    pub data: PathBuf,
}

impl SystemConfig {
    pub fn get_sqlite_path(&self) -> String {
        todo!()
    }
}
