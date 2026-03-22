use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Channels {
    Table,
    ChannelId,
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    MessageId,
    ChannelId,
    ExpiresAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channels::ChannelId)
                            .string()
                            .not_null()
                            .primary_key()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::MessageId)
                            .string()
                            .not_null()
                            .primary_key()
                            .to_owned(),
                    )
                    .col(string(Messages::ChannelId).not_null())
                    .col(date_time(Messages::ExpiresAt).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}
