use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Librarian;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/librarian", get(get_librarians).post(create_librarian))
        .route(
            "/librarian/:id",
            put(update_librarian).delete(delete_librarian),
        )
        .with_state(db)
}

async fn get_librarians(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Librarian>>), (StatusCode, String)> {
    let librarians = sqlx::query_as!(
        Librarian,
        r#"SELECT id, name, lastname, surname, age FROM librarian ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load librarians from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(librarians)))
}

async fn create_librarian(
    State(db): State<Pool<Postgres>>,
    Json(librarian): Json<Librarian>,
) -> Result<(StatusCode, Json<Librarian>), (StatusCode, String)> {
    let inserted_librarian = sqlx::query_as!(
        Librarian,
        r#"INSERT INTO librarian 
        (name, lastname, surname, age)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, lastname, surname, age"#,
        librarian.name,
        librarian.lastname,
        librarian.surname,
        librarian.age
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add librarian to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_librarian)))
}

async fn update_librarian(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(librarian): Json<Librarian>,
) -> Result<(StatusCode, Json<Librarian>), (StatusCode, String)> {
    tracing::info!("Librarian payload: {:?}", librarian);

    sqlx::query!(
        r#"UPDATE librarian SET
        name = $1,
        lastname = $2,
        surname = $3, 
        age = $4
        WHERE id = $5"#,
        librarian.name,
        librarian.lastname,
        librarian.surname,
        librarian.age,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update librarian in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(librarian)))
}

async fn delete_librarian(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Librarian>), (StatusCode, String)> {
    let deleted_librarian = sqlx::query_as!(
        Librarian,
        r#"DELETE FROM librarian WHERE id = $1 
    RETURNING id, name, lastname, surname, age"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update librarian in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_librarian)))
}
