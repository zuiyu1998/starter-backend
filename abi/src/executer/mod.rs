mod cmd;

use crate::{project::StarterProject, Result};
use std::process::Command;

use cmd::Cmd;

pub enum ExecuterKind {
    Cmd(Cmd),
}

impl ProjectExecuter for ExecuterKind {
    fn build(&self, project: StarterProject) -> Command {
        match self {
            ExecuterKind::Cmd(cmd) => cmd.build(project),
        }
    }
}

pub enum Executer {
    Kind(ExecuterKind),
    Custom,
}

impl ProjectExecuter for Executer {
    fn build(&self, project: StarterProject) -> Command {
        match self {
            Executer::Kind(kind) => kind.build(project),
            Executer::Custom => {
                todo!()
            }
        }
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
