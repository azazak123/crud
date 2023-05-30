use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::TeacherCard;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/teacher-card",
            get(get_teacher_cards).post(create_teacher_card),
        )
        .route(
            "/teacher-card/:id",
            put(update_teacher_card).delete(delete_teacher_card),
        )
        .with_state(db)
}

async fn get_teacher_cards(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<TeacherCard>>), (StatusCode, String)> {
    let teacher_cards = sqlx::query_as!(
        TeacherCard,
        r#"SELECT id, teacher, issue_date FROM teacher_card ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load teacher_cards from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teacher_cards)))
}

async fn create_teacher_card(
    State(db): State<Pool<Postgres>>,
    Json(teacher_card): Json<TeacherCard>,
) -> Result<(StatusCode, Json<TeacherCard>), (StatusCode, String)> {
    let inserted_teacher_card = sqlx::query_as!(
        TeacherCard,
        r#"INSERT INTO teacher_card 
        (teacher, issue_date)
        VALUES ($1, $2)
        RETURNING  id, teacher, issue_date"#,
        teacher_card.teacher,
        teacher_card.issue_date
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add teacher_card to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_teacher_card)))
}

async fn update_teacher_card(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(teacher_card): Json<TeacherCard>,
) -> Result<(StatusCode, Json<TeacherCard>), (StatusCode, String)> {
    tracing::info!("TeacherCard payload: {:?}", teacher_card);

    sqlx::query!(
        r#"UPDATE teacher_card SET
        teacher = $1,
        issue_date = $2
        WHERE id = $3"#,
        teacher_card.teacher,
        teacher_card.issue_date,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update teacher_card in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teacher_card)))
}

async fn delete_teacher_card(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<TeacherCard>), (StatusCode, String)> {
    let deleted_teacher_card = sqlx::query_as!(
        TeacherCard,
        r#"DELETE FROM teacher_card WHERE id = $1 
        RETURNING id, teacher, issue_date"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update teacher_card in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_teacher_card)))
}
