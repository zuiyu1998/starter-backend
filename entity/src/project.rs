use abi::{
    chrono::NaiveDateTime,
    project::{StarterProject, StarterProjectCreate, StarterProjectMeta, StarterProjectUpdate},
    sea_orm::{self, entity::prelude::*, IntoActiveModel, Set},
    utils::get_now,
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
    pub tags: String,
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
            &self.tags,
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
        active.tags = Set(meta.tags);

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
            tags: value.tags,
        };

        StarterProject {
            meta,
            executer: value.executer,
            id: value.id,
        }
    }
}

impl IntoActiveModel<ActiveModel> for StarterProjectUpdate {
    fn into_active_model(self) -> ActiveModel {
        let mut active: ActiveModel = Default::default();

        active.id = Set(self.id);

        let now: NaiveDateTime = get_now();

        if let Some(name) = self.name {
            active.name = Set(name);
        }

        if let Some(description) = self.description {
            active.description = Set(description);
        }

        active.update_at = Set(now);

        if let Some(path) = self.path {
            active.path = Set(path);
        }

        if let Some(exe_path) = self.exe_path {
            active.exe_path = Set(exe_path);
        }

        if let Some(icon) = self.icon {
            active.icon = Set(icon);
        }

        if let Some(tags) = self.tags {
            active.tags = Set(tags);
        }

        if let Some(executer) = self.executer {
            active.executer = Set(executer);
        }

        active
    }
}
