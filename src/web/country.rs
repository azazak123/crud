use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Country;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/country", get(get_countrys).post(create_country))
        .route("/country/:code", put(update_country).delete(delete_country))
        .with_state(db)
}

async fn get_countrys(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Country>>), (StatusCode, String)> {
    let countrys = sqlx::query_as!(
        Country,
        r#"SELECT code, name FROM country ORDER BY code ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load countrys from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(countrys)))
}

async fn create_country(
    State(db): State<Pool<Postgres>>,
    Json(country): Json<Country>,
) -> Result<(StatusCode, Json<Country>), (StatusCode, String)> {
    let inserted_country = sqlx::query_as!(
        Country,
        r#"INSERT INTO country 
        (code, name)
        VALUES ($1, $2)
        RETURNING code, name"#,
        country.code,
        country.name,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add country to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_country)))
}

async fn update_country(
    State(db): State<Pool<Postgres>>,
    Path(code): Path<String>,
    Json(country): Json<Country>,
) -> Result<(StatusCode, Json<Country>), (StatusCode, String)> {
    tracing::info!("Country payload: {:?}", country);

    sqlx::query!(
        r#"UPDATE country SET
        code = $1,
        name = $2
        WHERE code = $3"#,
        country.code,
        country.name,
        code
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update country in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(country)))
}

async fn delete_country(
    State(db): State<Pool<Postgres>>,
    Path(code): Path<String>,
) -> Result<(StatusCode, Json<Country>), (StatusCode, String)> {
    let deleted_country = sqlx::query_as!(
        Country,
        r#"DELETE FROM country WHERE code = $1 
        RETURNING code, name"#,
        code
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update country in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_country)))
}
