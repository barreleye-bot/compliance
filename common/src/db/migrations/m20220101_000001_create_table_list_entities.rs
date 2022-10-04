use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(ListEntities::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(ListEntities::ListEntityId)
							.big_integer()
							.not_null()
							.auto_increment()
							.primary_key(),
					)
					.col(
						ColumnDef::new(ListEntities::Id)
							.unique_key()
							.string()
							.not_null(),
					)
					.col(
						ColumnDef::new(ListEntities::Address)
							.unique_key()
							.string()
							.not_null(),
					)
					.col(
						ColumnDef::new(ListEntities::UpdatedAt)
							.date_time()
							.null(),
					)
					.col(
						ColumnDef::new(ListEntities::CreatedAt)
							.date_time()
							.not_null()
							.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(ListEntities::Table).to_owned())
			.await
	}
}

#[derive(Iden)]
enum ListEntities {
	#[iden = "list_entities"]
	Table,
	ListEntityId,
	Id,
	Address,
	UpdatedAt,
	CreatedAt,
}
