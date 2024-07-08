use super::ProjectExecuter;
use crate::project::StarterProjectMeta;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Clone, Deserialize, Serialize)]
pub enum Cmd {
    Path(CmdEnv),
}

impl ProjectExecuter for Cmd {
    fn build(&self, project: StarterProjectMeta) -> Command {
        match self {
            Cmd::Path(path) => path.build(project),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CmdEnv;

impl ProjectExecuter for CmdEnv {
    fn build(&self, project: StarterProjectMeta) -> Command {
        let args = format!("{} {}", project.exe_path, project.path);
        let mut command = Command::new("cmd");

        command.args(["/C", &args]);

        command
    }
}

mod test {

    #[test]
    fn test_cmd_path() {
        use super::CmdEnv;
        use crate::executer::ProjectExecuter;
        use crate::project::StarterProjectMeta;

        let project = StarterProjectMeta::new("code", ".", "", "", "", "test");

        let executer = CmdEnv;

        let res = executer.execute(project).ok().is_some();

        assert_eq!(res, true);
    }
}
