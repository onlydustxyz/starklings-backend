use crate::{
	course_config::{get_nft_id_for_section, get_uui_and_points_for_exercise},
	routes::{unwrap_option_or_return, unwrap_result_or_return},
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_auth::AuthBearer;
use octocrab::Octocrab;
use only_dust_contracts_api::ProfileId;
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
pub async fn submit_exercise(
	AuthBearer(token): AuthBearer,
	Json(payload): Json<SubmitExercisePayload>,
) -> impl IntoResponse {
	// Retrieve exercise related data from our static config
	let token_id = {
		let section_name = crate::routes::unwrap_option_or_return!(
			payload.exercise_path.split('/').next(),
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		);

		crate::routes::unwrap_option_or_return!(
			get_nft_id_for_section(section_name),
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		)
	};
	let (exercise_id, amount) = unwrap_option_or_return!(
		get_uui_and_points_for_exercise(&payload.exercise_path),
		StatusCode::INTERNAL_SERVER_ERROR.into_response()
	);

	// Get the OnlyDust profile id of the token bearer
	let profile_id: ProfileId = {
		// Get the github user id of the token bearer
		let user = {
			let octo = unwrap_result_or_return!(
				Octocrab::builder().personal_token(token).build(),
				StatusCode::BAD_REQUEST.into_response()
			);
			unwrap_result_or_return!(
				octo.current().user().await,
				StatusCode::BAD_REQUEST.into_response()
			)
		};

		// Query the OnlyDust registry for the profile id
		let raw_profile_id = unwrap_result_or_return!(
			only_dust_contracts_api::get_profile_id((*user.id).into()).await,
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		)
		.result;

		// Convert it to our custom type
		unwrap_result_or_return!(
			raw_profile_id.try_into(),
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		)
	};

	// Create a transaction to mint the token
	let add_transaction_result = {
		unwrap_result_or_return!(
			only_dust_contracts_api::mint_nft(profile_id, token_id, exercise_id, amount).await,
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		)
	};

	// Return the transaction hash so user can follow it on a block exporer
	(
		StatusCode::ACCEPTED,
		format!(
			"Transaction submitted, with hash `{}`",
			add_transaction_result.transaction_hash
		),
	)
		.into_response()
}
