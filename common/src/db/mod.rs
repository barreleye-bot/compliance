use eyre::Result;
use log::LevelFilter;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

mod migrations;
use migrations::{Migrator, MigratorTrait};

pub async fn new() -> Result<DatabaseConnection> {
	const URL: &str = "sqlite://barreleye-compliance.db?mode=rwc";

	let with_options = |url: String| -> ConnectOptions {
		let mut opt = ConnectOptions::new(url);
		opt.max_connections(100)
			.min_connections(5)
			.connect_timeout(Duration::from_secs(8))
			.idle_timeout(Duration::from_secs(8))
			.max_lifetime(Duration::from_secs(8))
			.sqlx_logging(false)
			.sqlx_logging_level(LevelFilter::Warn);

		opt
	};

	let db = Database::connect(with_options(URL.to_string())).await?;
	Migrator::up(&db, None).await?;

	Ok(db)
}
