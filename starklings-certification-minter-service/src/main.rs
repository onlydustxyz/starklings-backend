//! A service that Starklings certification tokens

mod application;
mod domain;
mod infrastructure;
mod routes;

use axum::{routing::post, Router};
use infrastructure::course_config::CourseConfig;
use routes::submit_exercise;
use std::{
	net::SocketAddr,
	sync::{Arc, LockResult, RwLock, RwLockReadGuard},
};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

#[derive(Debug, Clone)]
struct ReadOnlySharedState<T> {
	inner: Arc<RwLock<T>>,
}

impl<T> ReadOnlySharedState<T> {
	fn new(data: T) -> Self {
		Self {
			inner: Arc::new(RwLock::new(data)),
		}
	}

	fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
		self.inner.read()
	}
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let app = Router::new().route("/exercise/submit", post(submit_exercise)).layer(
		ServiceBuilder::new()
			.layer(AddExtensionLayer::new(ReadOnlySharedState::new(
				CourseConfig::read_from_toml(None),
			)))
			.into_inner(),
	);
	let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
