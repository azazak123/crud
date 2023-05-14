use axum::http::StatusCode;

pub fn internal_error(err: color_eyre::Report) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
