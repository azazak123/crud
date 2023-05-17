use axum::http::StatusCode;

pub fn internal_error(err: color_eyre::Report) -> (StatusCode, String) {
    tracing::error!("{:?}", err);

    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#}", err))
}
