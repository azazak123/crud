use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Faculty;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/faculty", get(get_facultys).post(create_faculty))
        .route("/faculty/:id", put(update_faculty).delete(delete_faculty))
        .with_state(db)
}

async fn get_facultys(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Faculty>>), (StatusCode, String)> {
    let facultys = sqlx::query_as!(
        Faculty,
        r#"SELECT id, name, letter FROM faculty ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load facultys from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(facultys)))
}

async fn create_faculty(
    State(db): State<Pool<Postgres>>,
    Json(faculty): Json<Faculty>,
) -> Result<(StatusCode, Json<Faculty>), (StatusCode, String)> {
    let inserted_faculty = sqlx::query_as!(
        Faculty,
        r#"INSERT INTO faculty 
        (name, letter)
        VALUES ($1, $2)
        RETURNING id, name, letter"#,
        faculty.name,
        faculty.letter,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add faculty to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_faculty)))
}

async fn update_faculty(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(faculty): Json<Faculty>,
) -> Result<(StatusCode, Json<Faculty>), (StatusCode, String)> {
    tracing::info!("Faculty payload: {:?}", faculty);

    sqlx::query!(
        r#"UPDATE faculty SET
        name = $1,
        letter = $2
        WHERE id = $3"#,
        faculty.name,
        faculty.letter,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update faculty in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(faculty)))
}

async fn delete_faculty(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Faculty>), (StatusCode, String)> {
    let deleted_faculty = sqlx::query_as!(
        Faculty,
        r#"DELETE FROM faculty WHERE id = $1 
        RETURNING id, name, letter"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update faculty in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_faculty)))
}
