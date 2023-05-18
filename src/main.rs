use std::net::SocketAddr;

use axum::{
    http::{header, Method},
    Router,
};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]);

    // build our application with a route
    let app = Router::new()
        .merge(web::table::routes(db_pool.clone()))
        .merge(web::student::routes(db_pool.clone()))
        .merge(web::author::routes(db_pool.clone()))
        .merge(web::book::routes(db_pool.clone()))
        .merge(web::category::routes(db_pool.clone()))
        .merge(web::author_book::routes(db_pool.clone()))
        .merge(web::teachers_borrowing::routes(db_pool.clone()))
        .merge(web::teacher_card::routes(db_pool.clone()))
        .merge(web::teacher::routes(db_pool.clone()))
        .merge(web::curriculum::routes(db_pool.clone()))
        .merge(web::faculty::routes(db_pool.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
