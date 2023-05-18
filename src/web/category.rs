use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Category;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/category", get(get_categorys).post(create_category))
        .route(
            "/category/:id",
            put(update_category).delete(delete_category),
        )
        .with_state(db)
}

async fn get_categorys(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Category>>), (StatusCode, String)> {
    let categorys = sqlx::query_as!(Category, r#"SELECT id, name FROM category ORDER BY id ASC"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load categorys from database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(categorys)))
}

async fn create_category(
    State(db): State<Pool<Postgres>>,
    Json(category): Json<Category>,
) -> Result<(StatusCode, Json<Category>), (StatusCode, String)> {
    let inserted_category = sqlx::query_as!(
        Category,
        r#"INSERT INTO category 
        (name)
        VALUES ($1)
        RETURNING id, name"#,
        category.name,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add category to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_category)))
}

async fn update_category(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(category): Json<Category>,
) -> Result<(StatusCode, Json<Category>), (StatusCode, String)> {
    tracing::info!("Category payload: {:?}", category);

    sqlx::query!(
        r#"UPDATE category SET
        name = $1
        WHERE id = $2"#,
        category.name,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update category in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(category)))
}

async fn delete_category(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Category>), (StatusCode, String)> {
    let deleted_category = sqlx::query_as!(
        Category,
        r#"DELETE FROM category WHERE id = $1 
        RETURNING id, name"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update category in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_category)))
}
