use std::net::SocketAddr;

use axum::{
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_url = dotenvy::var("DATABASE_URL").expect("Env variable `DATABASE_URL` should be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .wrap_err_with(|| eyre!("Unable connect to database"))?;

    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        .merge(web::table::routes(db_pool.clone()))
        .merge(web::student::routes(db_pool))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
