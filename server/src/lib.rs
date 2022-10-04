use axum::{routing::get, Router};
use eyre::Result;
use log::info;
use std::{net::SocketAddr, sync::Arc};

use barreleye_common::{db, AppState};

mod handlers;

#[tokio::main]
pub async fn start() -> Result<()> {
	let shared_state = Arc::new(AppState { db: db::new().await? });

	let app = Router::with_state(shared_state)
		.route("/v0/entities", get(handlers::v0::entities::get::handler));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	info!("Listening on {addr}…");
	axum::Server::bind(&addr).serve(app.into_make_service()).await?;

	Ok(())
}
