use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Users::Username).string_len(50).not_null().unique_key())
                    .col(ColumnDef::new(Users::Email).string_len(100).not_null().unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).string_len(255).not_null())
                    .col(ColumnDef::new(Users::DisplayName).string_len(100).null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Tags::UserId).integer().not_null())
                    .col(ColumnDef::new(Tags::Name).string_len(50).not_null())
                    .col(ColumnDef::new(Tags::Color).string_len(20).null())
                    .col(ColumnDef::new(Tags::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_user")
                            .from(Tags::Table, Tags::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tags_user_name")
                    .table(Tags::Table)
                    .col(Tags::UserId)
                    .col(Tags::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Wordbooks::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Wordbooks::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Wordbooks::UserId).integer().not_null())
                    .col(ColumnDef::new(Wordbooks::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Wordbooks::Description).text().null())
                    .col(ColumnDef::new(Wordbooks::CoverUrl).string_len(500).null())
                    .col(ColumnDef::new(Wordbooks::SortOrder).integer().not_null().default(0))
                    .col(ColumnDef::new(Wordbooks::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Wordbooks::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wordbooks_user")
                            .from(Wordbooks::Table, Wordbooks::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wordbooks_user")
                    .table(Wordbooks::Table)
                    .col(Wordbooks::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Chapters::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Chapters::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Chapters::WordbookId).integer().not_null())
                    .col(ColumnDef::new(Chapters::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Chapters::SortOrder).integer().not_null().default(0))
                    .col(ColumnDef::new(Chapters::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Chapters::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chapters_wordbook")
                            .from(Chapters::Table, Chapters::WordbookId)
                            .to(Wordbooks::Table, Wordbooks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chapters_wordbook")
                    .table(Chapters::Table)
                    .col(Chapters::WordbookId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Words::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Words::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Words::ChapterId).integer().not_null())
                    .col(ColumnDef::new(Words::Source).string_len(500).not_null())
                        .col(ColumnDef::new(Words::Translation).string_len(500).not_null())
                        .col(ColumnDef::new(Words::Note).text().null())
                    .col(ColumnDef::new(Words::SortOrder).integer().not_null().default(0))
                    .col(ColumnDef::new(Words::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Words::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_words_chapter")
                            .from(Words::Table, Words::ChapterId)
                            .to(Chapters::Table, Chapters::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_words_chapter")
                    .table(Words::Table)
                    .col(Words::ChapterId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WordTags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WordTags::WordId).integer().not_null())
                    .col(ColumnDef::new(WordTags::TagId).integer().not_null())
                    .primary_key(Index::create().col(WordTags::WordId).col(WordTags::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_word_tags_word")
                            .from(WordTags::Table, WordTags::WordId)
                            .to(Words::Table, Words::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_word_tags_tag")
                            .from(WordTags::Table, WordTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_word_tags_tag")
                    .table(WordTags::Table)
                    .col(WordTags::TagId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(WordTags::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Words::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Chapters::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Wordbooks::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Tags::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    DisplayName,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Tags {
    Table,
    Id,
    UserId,
    Name,
    Color,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Wordbooks {
    Table,
    Id,
    UserId,
    Name,
    Description,
    CoverUrl,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Chapters {
    Table,
    Id,
    WordbookId,
    Name,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Words {
    Table,
    Id,
    ChapterId,
    Source,
    Translation,
    Note,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum WordTags {
    Table,
    WordId,
    TagId,
}