use sea_orm::DatabaseConnection;

pub mod constants;
pub mod db;
pub mod models;
pub mod utils;

pub struct AppState {
	pub db: DatabaseConnection,
}
