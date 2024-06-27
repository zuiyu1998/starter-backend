use entity::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::Uuid)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectColumn::Path).string().not_null())
                    .col(ColumnDef::new(ProjectColumn::ExePath).string().not_null())
                    .col(ColumnDef::new(ProjectColumn::Icon).string().not_null())
                    .col(ColumnDef::new(ProjectColumn::Name).string().not_null())
                    .col(ColumnDef::new(ProjectColumn::Executer).integer().not_null())
                    .col(
                        ColumnDef::new(ProjectColumn::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::IsDelete)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::IsEnable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectEntity).to_owned())
            .await
    }
}
