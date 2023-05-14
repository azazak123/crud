use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/table", get(get_tables))
        .with_state(db)
}

async fn get_tables(
    State(db): State<Pool<Postgres>>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let tables = sqlx::query!(
        "SELECT TABLE_NAME FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load tables from database"))
    .map_err(internal_error)?
    .into_iter()
    .filter_map(|v| v.table_name)
    .collect();

    Ok(Json(tables))
}
