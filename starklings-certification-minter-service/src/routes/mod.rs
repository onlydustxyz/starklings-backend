pub mod submit_exercise;
pub use submit_exercise::*;

use axum::{http::StatusCode, response::IntoResponse};

use crate::domain::errors::Error;

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
	}
}
