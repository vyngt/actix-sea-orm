use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_course"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Course::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Course::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Course::TutorId).integer().not_null())
                    .col(ColumnDef::new(Course::Name).string().not_null())
                    .col(
                        ColumnDef::new(Course::PostedTime)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Course::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Course {
    Table,
    Id,
    TutorId,
    Name,
    PostedTime,
}
