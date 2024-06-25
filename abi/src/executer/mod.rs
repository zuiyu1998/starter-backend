mod cmd;

use crate::{project::StarterProjectMeta, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

pub use cmd::{Cmd, CmdPath};

#[derive(Clone, Deserialize, Serialize)]

pub enum ExecuterKind {
    Cmd(Cmd),
}

impl ProjectExecuter for ExecuterKind {
    fn build(&self, project: StarterProjectMeta) -> Command {
        match self {
            ExecuterKind::Cmd(cmd) => cmd.build(project),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]

pub enum Executer {
    Kind(ExecuterKind),
    Custom,
}

impl From<CmdPath> for Executer {
    fn from(value: CmdPath) -> Self {
        Executer::Kind(ExecuterKind::Cmd(Cmd::Path(value)))
    }
}

impl ProjectExecuter for Executer {
    fn build(&self, project: StarterProjectMeta) -> Command {
        match self {
            Executer::Kind(kind) => kind.build(project),
            Executer::Custom => {
                todo!()
            }
        }
    }
}

pub trait ProjectExecuter: 'static + Send + Sync {
    fn build(&self, project: StarterProjectMeta) -> Command;

    fn execute(&self, project: StarterProjectMeta) -> Result<()> {
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
