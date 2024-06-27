use abi::{
    chrono::NaiveDateTime,
    project::{into_executer, StarterProject, StarterProjectCreate, StarterProjectMeta},
    sea_orm::{self, entity::prelude::*, IntoActiveModel, Set},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project")]
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
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for StarterProjectCreate {
    fn into_active_model(self) -> ActiveModel {
        let meta = StarterProjectMeta::new(
            &self.exe_path,
            &self.path,
            &self.icon,
            &self.name,
            &self.description,
        );

        let mut active: ActiveModel = Default::default();

        active.name = Set(meta.name);
        active.description = Set(meta.description);
        active.uuid = Set(meta.uuid);
        active.create_at = Set(meta.create_at);
        active.update_at = Set(meta.update_at);
        active.path = Set(meta.path);
        active.exe_path = Set(meta.exe_path);
        active.icon = Set(meta.icon);

        active.executer = Set(self.executer);

        active
    }
}

impl From<Model> for StarterProject {
    fn from(value: Model) -> Self {
        let meta = StarterProjectMeta {
            uuid: value.uuid,
            path: value.path,
            exe_path: value.exe_path,
            create_at: value.create_at,
            update_at: value.update_at,
            icon: value.icon,
            name: value.name,
            description: value.description,
        };

        let executer = into_executer(value.executer);

        StarterProject { meta, executer }
    }
}
