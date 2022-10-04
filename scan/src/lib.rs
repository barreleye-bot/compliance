use eyre::Result;
use log::info;

use barreleye_common::{
	db,
	models::{list_entity::ListEntity, BasicModel},
};

mod list;

#[tokio::main]
pub async fn start() -> Result<()> {
	let database = db::new().await?;

	if let Ok(addresses) = list::get_addresses().await {
		for address in addresses.into_iter() {
			ListEntity::try_create(&database, ListEntity::new_model(&address)?)
				.await?;
		}

		let total_addresses = ListEntity::count_all(&database).await?;
		info!("Updated");
		info!("Database contains {total_addresses} address(es)");
	} else {
		info!("Could not fetch data");
	}

	Ok(())
}
