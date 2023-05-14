use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::{Student, StudentStatus};

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/student", get(get_students))
        .with_state(db)
}

async fn get_students(
    State(db): State<Pool<Postgres>>,
) -> Result<Json<Vec<Student>>, (StatusCode, String)> {
    let students = sqlx::query_as!(Student, r#"SELECT id, name, lastname, surname, age, faculty_curriculum, "group", start_study_date, status as "status: StudentStatus" FROM student"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load students from database"))
        .map_err(internal_error)?;

    Ok(Json(students))
}
