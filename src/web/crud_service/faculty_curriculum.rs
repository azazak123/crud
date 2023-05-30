use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::FacultyCurriculum;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/faculty-curriculum",
            get(get_faculty_curriculums).post(create_faculty_curriculum),
        )
        .route(
            "/faculty-curriculum/:id",
            put(update_faculty_curriculum).delete(delete_faculty_curriculum),
        )
        .with_state(db)
}

async fn get_faculty_curriculums(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<FacultyCurriculum>>), (StatusCode, String)> {
    let faculty_curriculums = sqlx::query_as!(
        FacultyCurriculum,
        r#"SELECT id, faculty, curriculum FROM faculty_curriculum ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load faculty_curriculums from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(faculty_curriculums)))
}

async fn create_faculty_curriculum(
    State(db): State<Pool<Postgres>>,
    Json(faculty_curriculum): Json<FacultyCurriculum>,
) -> Result<(StatusCode, Json<FacultyCurriculum>), (StatusCode, String)> {
    let inserted_faculty_curriculum = sqlx::query_as!(
        FacultyCurriculum,
        r#"INSERT INTO faculty_curriculum 
        (faculty, curriculum)
        VALUES ($1, $2)
        RETURNING id, faculty, curriculum"#,
        faculty_curriculum.faculty,
        faculty_curriculum.curriculum,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add faculty_curriculum to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_faculty_curriculum)))
}

async fn update_faculty_curriculum(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(faculty_curriculum): Json<FacultyCurriculum>,
) -> Result<(StatusCode, Json<FacultyCurriculum>), (StatusCode, String)> {
    tracing::info!("FacultyCurriculum payload: {:?}", faculty_curriculum);

    sqlx::query!(
        r#"UPDATE faculty_curriculum SET
        faculty = $1,
        curriculum = $2
        WHERE id = $3"#,
        faculty_curriculum.faculty,
        faculty_curriculum.curriculum,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update faculty_curriculum in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(faculty_curriculum)))
}

async fn delete_faculty_curriculum(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<FacultyCurriculum>), (StatusCode, String)> {
    let deleted_faculty_curriculum = sqlx::query_as!(
        FacultyCurriculum,
        r#"DELETE FROM faculty_curriculum WHERE id = $1 
        RETURNING id, faculty, curriculum"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update faculty_curriculum in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_faculty_curriculum)))
}
