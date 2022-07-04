use crate::{
	domain::errors::Error, infrastructure::course_config::CourseConfig, ReadOnlySharedState,
};
use axum::{extract::Extension, http::StatusCode, Json};
use axum_auth::AuthBearer;
use axum_macros::debug_handler;
use lazy_static::__Deref;
use serde::Deserialize;

/// Body expected by `submit_exercise`
#[derive(Deserialize)]
pub struct SubmitExercisePayload {
	exercise_path: String,
}

/// Mint the points the user earned for completing the exercise
///
/// # Flow
/// 1) Ask Github for the github_id of the AuthBearer token
/// 2) Ensure this user have set a `ProfileId` in OnlyDust registry contract
/// 3) Mint an amount of soulbound token to this ProfileId
///
/// # Bearer Authentication
/// a github authentication token is required
///
/// # Body
/// `exercise_path` is a string respecting this format "<section_name>/<exercise_name>"
#[debug_handler]
pub(crate) async fn submit_exercise(
	AuthBearer(token): AuthBearer,
	Json(payload): Json<SubmitExercisePayload>,
	Extension(shared_course_config): Extension<ReadOnlySharedState<CourseConfig>>,
) -> Result<(StatusCode, String), Error> {
	// Retrieve exercise related data from our static config
	let (exercise_id, token_id, points) = crate::application::get_exercise_data(
		&payload.exercise_path,
		// This is safe because shared_course_config cannot be written to
		unsafe { shared_course_config.read().unwrap_unchecked() }.deref(),
	)?;

	// Get the OnlyDust profile id of the token bearer
	let profile_id = crate::application::get_profile_id(token).await?;

	// Create a transaction to mint the token
	let transaction_hash =
		crate::application::mint_nft(profile_id, token_id, exercise_id, points).await?;

	// Return the transaction hash so user can follow it on a block exporer
	Ok((StatusCode::OK, transaction_hash.to_string()))
}
