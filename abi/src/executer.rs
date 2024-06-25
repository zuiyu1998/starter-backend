use crate::{project::StarterProject, Result};
use std::process::Command;

pub struct Executer(Box<dyn ProjectExecuter>);

impl ProjectExecuter for Executer {
    fn build(&self, project: StarterProject) -> Command {
        self.0.build(project)
    }
}

pub trait ProjectExecuter: 'static + Send + Sync {
    fn build(&self, project: StarterProject) -> Command;

    fn execute(&self, project: StarterProject) -> Result<()> {
        let mut command = self.build(project);

        match command.spawn() {
            Ok(mut child) => {
                child.wait()?;
            }

            Err(e) => {
                tracing::error!("command spawn error :{}", e);
            }
        }

        Ok(())
    }
}
