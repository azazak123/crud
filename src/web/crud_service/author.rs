use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Author;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/author", get(get_authors).post(create_author))
        .route("/author/:id", put(update_author).delete(delete_author))
        .with_state(db)
}

async fn get_authors(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let authors = sqlx::query_as!(
        Author,
        r#"SELECT id, name, lastname, surname, country FROM author ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load authors from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(authors)))
}

async fn create_author(
    State(db): State<Pool<Postgres>>,
    Json(author): Json<Author>,
) -> Result<(StatusCode, Json<Author>), (StatusCode, String)> {
    let inserted_author = sqlx::query_as!(
        Author,
        r#"INSERT INTO author 
        (name, lastname, surname, country)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, lastname, surname, country"#,
        author.name,
        author.lastname,
        author.surname,
        author.country,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add author to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_author)))
}

async fn update_author(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(author): Json<Author>,
) -> Result<(StatusCode, Json<Author>), (StatusCode, String)> {
    tracing::info!("author payload: {:?}", author);

    sqlx::query!(
        r#"UPDATE author SET
        name = $1,
        lastname = $2,
        surname = $3, 
        country = $4
        WHERE id = $5"#,
        author.name,
        author.lastname,
        author.surname,
        author.country,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update author in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(author)))
}

async fn delete_author(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Author>), (StatusCode, String)> {
    let deleted_author = sqlx::query_as!(
        Author,
        r#"DELETE FROM author WHERE id = $1 
    RETURNING id, name, lastname, surname, country"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update author in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_author)))
}
