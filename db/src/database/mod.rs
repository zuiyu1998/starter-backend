mod dao;
mod project;

use crate::Result;
use abi::{config::Config, sea_orm::Database};
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
        let conn = Database::connect(&config.system.get_sqlite_path()).await?;

        Migrator::up(&conn, None).await?;

        Ok(Db {
            project: Arc::new(DaoPoject::new(conn.clone())),
        })
    }
}
