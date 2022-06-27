//! A service that Starklings certification tokens

#[macro_use]
extern crate lazy_static;

mod course_config;
mod routes;

use axum::{routing::post, Router};
use routes::submit_exercise;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let app = Router::new().route("/exercise/submit", post(submit_exercise));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
