mod dao;
mod project;

use dao::DaoPoject;
use project::ProjectRepo;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    project: Arc<dyn ProjectRepo>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            project: Arc::new(DaoPoject),
        }
    }
}
