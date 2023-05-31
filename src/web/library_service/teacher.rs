use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::TeacherReadonly;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/teacher-readonly", get(get_teachers))
        .with_state(db)
}

async fn get_teachers(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<TeacherReadonly>>), (StatusCode, String)> {
    let teachers = sqlx::query_as!(
        TeacherReadonly,
        r#"SELECT t.id, t.name, t.lastname, t.surname, t.age,
        f.name as "faculty!",
        t.status as "status: _" FROM
        teacher as t
        LEFT JOIN faculty as f ON (f.id = t.faculty)
        ORDER BY t.id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load students from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teachers)))
}
