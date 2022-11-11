use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MemberInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MemberInfo::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MemberInfo::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(MemberInfo::CreatedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(MemberInfo::CreatedId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MemberInfo::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(MemberInfo::UpdatedId).big_integer())
                    .col(ColumnDef::new(MemberInfo::Username).string().not_null())
                    .col(ColumnDef::new(MemberInfo::DisplayName).string().not_null())
                    .col(ColumnDef::new(MemberInfo::Password).string())
                    .col(ColumnDef::new(MemberInfo::SignUpType).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(MemberInfo::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum MemberInfo {
    Table,
    Id,
    Active,
    CreatedAt,
    CreatedId,
    UpdatedAt,
    UpdatedId,
    Username,
    DisplayName,
    Password,
    SignUpType,
}
