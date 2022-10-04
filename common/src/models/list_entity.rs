use eyre::Result;
use sea_orm::entity::{prelude::*, *};
use sea_orm_migration::prelude::OnConflict;
use serde::{Deserialize, Serialize};

use crate::{constants::PREFIX_LIST_ENTITY_ID, models::BasicModel, utils};

#[derive(
	Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "list_entities")]
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub list_entity_id: i64,
	pub id: String,
	pub address: String,
	#[sea_orm(nullable)]
	pub updated_at: Option<DateTime>,
	pub created_at: DateTime,
}

pub use ActiveModel as ListEntityActiveModel;
pub use Model as ListEntity;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		panic!("No RelationDef")
	}
}

impl ActiveModelBehavior for ActiveModel {}

impl BasicModel for Model {
	type ActiveModel = ActiveModel;
}

impl Model {
	pub fn new_model(address: &str) -> Result<ActiveModel> {
		Ok(ActiveModel {
			id: Set(utils::new_unique_id(PREFIX_LIST_ENTITY_ID)),
			address: Set(address.to_string()),
			..Default::default()
		})
	}

	pub async fn try_create(
		db: &DatabaseConnection,
		active_model: ActiveModel,
	) -> Result<i64> {
		let res = Entity::insert(active_model)
			.on_conflict(
				OnConflict::column(Column::Address).do_nothing().to_owned(),
			)
			.exec(db)
			.await?;

		Ok(res.last_insert_id)
	}

	pub async fn get_by_address(
		db: &DatabaseConnection,
		address: &str,
	) -> Result<Option<Self>> {
		let list_entity = Entity::find()
			.filter(Column::Address.eq(address.to_lowercase()))
			.one(db)
			.await?;

		Ok(list_entity)
	}
}
