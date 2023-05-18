use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Teacher;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/teacher", get(get_teachers).post(create_teacher))
        .route("/teacher/:id", put(update_teacher).delete(delete_teacher))
        .with_state(db)
}

async fn get_teachers(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Teacher>>), (StatusCode, String)> {
    let teachers = sqlx::query_as!(Teacher,
         r#"SELECT id, name, lastname, surname, age, faculty, status as "status: _" FROM teacher ORDER BY id ASC"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load teachers from database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teachers)))
}

async fn create_teacher(
    State(db): State<Pool<Postgres>>,
    Json(teacher): Json<Teacher>,
) -> Result<(StatusCode, Json<Teacher>), (StatusCode, String)> {
    let inserted_teacher = sqlx::query_as!(
        Teacher,
        r#"INSERT INTO teacher 
        (name, lastname, surname, age, faculty, status)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, lastname, surname, age, faculty, status as "status: _""#,
        teacher.name,
        teacher.lastname,
        teacher.surname,
        teacher.age,
        teacher.faculty,
        teacher.status as _
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add teacher to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_teacher)))
}

async fn update_teacher(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(teacher): Json<Teacher>,
) -> Result<(StatusCode, Json<Teacher>), (StatusCode, String)> {
    tracing::info!("Teacher payload: {:?}", teacher);

    sqlx::query!(
        r#"UPDATE teacher SET
        name = $1,
        lastname = $2,
        surname = $3, 
        age = $4, 
        faculty = $5, 
        status = $6
        WHERE id = $7"#,
        teacher.name,
        teacher.lastname,
        teacher.surname,
        teacher.age,
        teacher.faculty,
        teacher.status as _,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update teacher in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teacher)))
}

async fn delete_teacher(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Teacher>), (StatusCode, String)> {
    let deleted_teacher = sqlx::query_as!(
        Teacher,
        r#"DELETE FROM teacher WHERE id = $1 
        RETURNING id, name, lastname, surname, age, faculty, status as "status: _""#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update teacher in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_teacher)))
}
