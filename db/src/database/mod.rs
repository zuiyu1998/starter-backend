mod dao;
mod project;

use crate::Result;
use abi::{
    config::Config,
    sea_orm::Database,
    tokio::fs::{self},
};
use dao::DaoPoject;
use project::ProjectRepo;
use std::sync::Arc;

use migration::{Migrator, MigratorTrait};

#[derive(Clone)]
pub struct Db {
    pub project: Arc<dyn ProjectRepo>,
}

impl Db {
    pub async fn from_config(config: &Config) -> Result<Self> {
        //创建文件夹
        fs::create_dir_all(config.system.get_db_dir())
            .await
            .expect("db dir create fail.");

        //创建文件
        let db_path = config.system.get_sqlite_path();
        if !fs::try_exists(&db_path).await? {
            fs::File::create(&db_path).await?;
        }

        let conn_str = format!("sqlite:///{}", db_path.to_string_lossy());

        let conn = Database::connect(&conn_str).await?;

        Migrator::up(&conn, None).await?;

        Ok(Db {
            project: Arc::new(DaoPoject::new(conn.clone())),
        })
    }
}
