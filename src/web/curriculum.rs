use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Curriculum;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/curriculum", get(get_curriculums).post(create_curriculum))
        .route(
            "/curriculum/:id",
            put(update_curriculum).delete(delete_curriculum),
        )
        .with_state(db)
}

async fn get_curriculums(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Curriculum>>), (StatusCode, String)> {
    let curriculums = sqlx::query_as!(
        Curriculum,
        r#"SELECT id, name, letter FROM curriculum ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load curriculums from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(curriculums)))
}

async fn create_curriculum(
    State(db): State<Pool<Postgres>>,
    Json(curriculum): Json<Curriculum>,
) -> Result<(StatusCode, Json<Curriculum>), (StatusCode, String)> {
    let inserted_curriculum = sqlx::query_as!(
        Curriculum,
        r#"INSERT INTO curriculum 
        (name, letter)
        VALUES ($1, $2)
        RETURNING id, name, letter"#,
        curriculum.name,
        curriculum.letter,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add curriculum to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_curriculum)))
}

async fn update_curriculum(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(curriculum): Json<Curriculum>,
) -> Result<(StatusCode, Json<Curriculum>), (StatusCode, String)> {
    tracing::info!("Curriculum payload: {:?}", curriculum);

    sqlx::query!(
        r#"UPDATE curriculum SET
        name = $1,
        letter = $2
        WHERE id = $3"#,
        curriculum.name,
        curriculum.letter,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update curriculum in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(curriculum)))
}

async fn delete_curriculum(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Curriculum>), (StatusCode, String)> {
    let deleted_curriculum = sqlx::query_as!(
        Curriculum,
        r#"DELETE FROM curriculum WHERE id = $1 
        RETURNING id, name, letter"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update curriculum in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_curriculum)))
}
