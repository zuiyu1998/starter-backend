use super::ProjectExecuter;

use crate::project::StarterProjectMeta;
use std::process::Command;

pub enum Cmd {
    Path(CmdPath),
}

impl ProjectExecuter for Cmd {
    fn build(&self, project: StarterProjectMeta) -> Command {
        match self {
            Cmd::Path(path) => path.build(project),
        }
    }
}

pub struct CmdPath;

impl ProjectExecuter for CmdPath {
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
        use super::CmdPath;
        use crate::executer::ProjectExecuter;
        use crate::project::StarterProjectMeta;

        let project = StarterProjectMeta::new("code", ".", "");

        let executer = CmdPath;

        let res = executer.execute(project).ok().is_some();

        assert_eq!(res, true);
    }
}
