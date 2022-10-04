use axum::{
	extract::{Query, State},
	response::IntoResponse,
	Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use barreleye_common::{models::ListEntity, AppState};

#[derive(Deserialize)]
pub struct Payload {
	address: String,
}

#[derive(Serialize)]
pub struct Response {
	address: String,
	sanctioned: bool,
}

pub async fn handler(
	State(app): State<Arc<AppState>>,
	Query(payload): Query<Payload>,
) -> impl IntoResponse {
	let mut response =
		Response { address: payload.address.clone(), sanctioned: false };

	if ListEntity::get_by_address(&app.db, &payload.address)
		.await
		.unwrap()
		.is_some()
	{
		response.sanctioned = true;
	}

	Json(response)
}
