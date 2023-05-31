use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::StudentWithGroup;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/student-group", get(get_students))
        .with_state(db)
}

async fn get_students(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<StudentWithGroup>>), (StatusCode, String)> {
    let students = sqlx::query_as!(StudentWithGroup,
        r#"SELECT s.id, s.name, s.lastname, s.surname, s.age,
        f.letter || c.letter || '-' || RIGHT(TO_CHAR(s.start_study_date, 'yyyy'), 2) || '-' || s."group" as "group!",
        s.start_study_date, s.status as "status: _"  FROM
        student as s
        LEFT JOIN faculty_curriculum as fc ON (s.faculty_curriculum=fc.id) 
        LEFT JOIN faculty as f ON (f.id = fc.faculty)
        LEFT JOIN curriculum as c ON (c.id = fc.curriculum) ORDER BY s.id ASC"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load students from database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(students)))
}
