use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::StudentCard;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/student-card",
            get(get_student_cards).post(create_student_card),
        )
        .route(
            "/student-card/:id",
            put(update_student_card).delete(delete_student_card),
        )
        .with_state(db)
}

async fn get_student_cards(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<StudentCard>>), (StatusCode, String)> {
    let student_cards = sqlx::query_as!(
        StudentCard,
        r#"SELECT id, student, issue_date FROM student_card ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load student_cards from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(student_cards)))
}

async fn create_student_card(
    State(db): State<Pool<Postgres>>,
    Json(student_card): Json<StudentCard>,
) -> Result<(StatusCode, Json<StudentCard>), (StatusCode, String)> {
    let inserted_student_card = sqlx::query_as!(
        StudentCard,
        r#"INSERT INTO student_card 
        (student, issue_date)
        VALUES ($1, $2)
        RETURNING  id, student, issue_date"#,
        student_card.student,
        student_card.issue_date
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add student_card to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_student_card)))
}

async fn update_student_card(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(student_card): Json<StudentCard>,
) -> Result<(StatusCode, Json<StudentCard>), (StatusCode, String)> {
    tracing::info!("StudentCard payload: {:?}", student_card);

    sqlx::query!(
        r#"UPDATE student_card SET
        student = $1,
        issue_date = $2
        WHERE id = $3"#,
        student_card.student,
        student_card.issue_date,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update student_card in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(student_card)))
}

async fn delete_student_card(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<StudentCard>), (StatusCode, String)> {
    let deleted_student_card = sqlx::query_as!(
        StudentCard,
        r#"DELETE FROM student_card WHERE id = $1 
        RETURNING id, student, issue_date"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update student_card in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_student_card)))
}
