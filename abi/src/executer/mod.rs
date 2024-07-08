mod cmd;

use crate::{project::StarterProjectMeta, Result};
use cmd::Binary;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, process::Command};

pub use cmd::{Cmd, CmdEnv};

impl Display for Executer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Executer::Cmd(kind) => match kind {
                Cmd::Env(_) => f.write_str("cmd env"),
                Cmd::Binary(_) => f.write_str("cmd binary"),
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]

pub enum Executer {
    Cmd(Cmd),
}

impl From<CmdEnv> for Executer {
    fn from(value: CmdEnv) -> Self {
        Executer::Cmd(Cmd::Env(value))
    }
}

impl From<Binary> for Executer {
    fn from(value: Binary) -> Self {
        Executer::Cmd(Cmd::Binary(value))
    }
}

impl ProjectExecuter for Executer {
    fn build(&self, project: StarterProjectMeta) -> Command {
        match self {
            Executer::Cmd(kind) => kind.build(project),
        }
    }
}

impl Executer {
    pub fn as_i32(&self) -> i32 {
        match self {
            Executer::Cmd(kind) => match kind {
                Cmd::Binary(_) => 1,
                Cmd::Env(_) => 2,
            },
        }
    }
}

impl From<i32> for Executer {
    fn from(value: i32) -> Self {
        match value {
            1 => Executer::from(Binary),
            2 => Executer::from(CmdEnv),
            _ => Executer::from(Binary),
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
