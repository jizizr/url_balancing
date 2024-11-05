use sea_orm::Schema;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        manager
            .create_table(
                schema
                    .create_table_from_entity(entity::key::Entity)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        for mut index in schema
            .create_index_from_entity(entity::key::Entity)
            .to_owned()
        {
            manager
                .create_index(index.if_not_exists().to_owned())
                .await?;
        }

        manager
            .create_table(
                schema
                    .create_table_from_entity(entity::url::Entity)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entity::key::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(entity::url::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
