use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Publisher;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/publisher", get(get_publishers).post(create_publisher))
        .route(
            "/publisher/:id",
            put(update_publisher).delete(delete_publisher),
        )
        .with_state(db)
}

async fn get_publishers(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Publisher>>), (StatusCode, String)> {
    let publishers = sqlx::query_as!(
        Publisher,
        r#"SELECT id, name, country FROM publisher ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load publishers from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(publishers)))
}

async fn create_publisher(
    State(db): State<Pool<Postgres>>,
    Json(publisher): Json<Publisher>,
) -> Result<(StatusCode, Json<Publisher>), (StatusCode, String)> {
    let inserted_publisher = sqlx::query_as!(
        Publisher,
        r#"INSERT INTO publisher 
        (name, country)
        VALUES ($1, $2)
        RETURNING id, name, country"#,
        publisher.name,
        publisher.country,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add publisher to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_publisher)))
}

async fn update_publisher(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(publisher): Json<Publisher>,
) -> Result<(StatusCode, Json<Publisher>), (StatusCode, String)> {
    tracing::info!("Publisher payload: {:?}", publisher);

    sqlx::query!(
        r#"UPDATE publisher SET
        name = $1,
        country = $2
        WHERE id = $3"#,
        publisher.name,
        publisher.country,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update publisher in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(publisher)))
}

async fn delete_publisher(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Publisher>), (StatusCode, String)> {
    let deleted_publisher = sqlx::query_as!(
        Publisher,
        r#"DELETE FROM publisher WHERE id = $1 
        RETURNING id, name, country"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update publisher in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_publisher)))
}
