mod cmd;

use crate::{project::StarterProjectMeta, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, process::Command};

pub use cmd::{Cmd, CmdEnv};

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

impl Display for Executer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Executer::Custom => f.write_str("custom"),
            Executer::Kind(kind) => match kind {
                ExecuterKind::Cmd(cmd) => match cmd {
                    Cmd::Path(_) => f.write_str("cmd env"),
                },
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]

pub enum Executer {
    Kind(ExecuterKind),
    Custom,
}

impl From<CmdEnv> for Executer {
    fn from(value: CmdEnv) -> Self {
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

impl Executer {
    pub fn as_i32(&self) -> i32 {
        match self {
            Executer::Custom => 1,
            Executer::Kind(kind) => match kind {
                ExecuterKind::Cmd(cmd) => match cmd {
                    Cmd::Path(_) => 2,
                },
            },
        }
    }
}

impl From<i32> for Executer {
    fn from(value: i32) -> Self {
        match value {
            1 => Executer::Custom,
            2 => Executer::from(CmdEnv),
            _ => Executer::Custom,
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
