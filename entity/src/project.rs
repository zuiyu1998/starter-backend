use abi::{
    chrono::NaiveDateTime,
    executer::{Cmd, CmdPath, Executer, ExecuterKind},
    sea_orm::{self, entity::prelude::*},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: Uuid,
    pub path: String,
    pub exe_path: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub executer: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn from_executer(executer: Executer) -> i32 {
    match executer {
        Executer::Custom => 1,
        Executer::Kind(kind) => match kind {
            ExecuterKind::Cmd(cmd) => match cmd {
                Cmd::Path(_) => 2,
            },
        },
    }
}

fn into_executer(num: i32) -> Option<Executer> {
    match num {
        1 => Some(Executer::Custom),
        2 => Some(Executer::from(CmdPath)),
        _ => None,
    }
}
