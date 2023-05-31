use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::CardReadonly;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/card-readonly", get(get_cards))
        .with_state(db)
}

async fn get_cards(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<CardReadonly>>), (StatusCode, String)> {
    let cards = sqlx::query_as!(
        CardReadonly,
        r#"SELECT c.id as "id!", 
        s.lastname || ' ' || s.name || ' ' || s.surname as "owner!",
        c.issue_date as "issue_date!", false as "is_teacher!"
        FROM student_card as c 
        JOIN student as s ON s.id = c.student 
        UNION SELECT c.id as "id!", 
        t.lastname || ' ' || t.name || ' ' || t.surname as "owner!",
        c.issue_date as "issue_date!", true as "is_teacher!"
        FROM teacher_card as c
        JOIN teacher as t ON t.id = c.teacher"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load cards from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(cards)))
}
