use crate::{
	domain::{
		errors::Error,
		traits::{CourseConfigGetter, GithubUserGetter},
	},
	infrastructure::{course_config::CourseConfig, octocrab_implem::OctocrabInstance},
};
use only_dust_contracts_api::ProfileId;
use starknet::core::types::FieldElement;

pub fn get_exercise_data(
	exercise_path: &str,
	course_config: &CourseConfig,
) -> Result<(FieldElement, FieldElement, FieldElement), Error> {
	let section_name = exercise_path
		.split('/')
		.next()
		.ok_or_else(|| Error::InvalidExercisePath(exercise_path.to_string()))?;
	let token_id = course_config
		.get_nft_id_for_section(section_name)
		.ok_or_else(|| Error::UnknownSection(section_name.to_string()))?;

	let (exercise_id, points) = course_config
		.get_id_and_points_for_exercise(exercise_path)
		.ok_or_else(|| Error::UnknownExercise(exercise_path.to_string()))?;

	Ok((exercise_id, token_id, points))
}

pub async fn get_profile_id(token: String) -> Result<ProfileId, Error> {
	let octocrab_instance =
		OctocrabInstance::new(token.clone()).map_err(|e| Error::Chain(e.to_string()))?;

	let github_id = octocrab_instance
		.get_user_id(&token)
		.await
		.map_err(|e| Error::Chain(e.to_string()))?;

	let profile_id = only_dust_contracts_api::get_onlydust_profile_for_github_user(
		FieldElement::from(github_id),
	)
	.await
	.map_err(|e| Error::Chain(e.to_string()))?;

	Ok(profile_id)
}

pub async fn mint_nft(
	recipient_id: ProfileId,
	token_id: FieldElement,
	exercise_id: FieldElement,
	amount: FieldElement,
) -> Result<FieldElement, Error> {
	only_dust_contracts_api::increase_certification_score(
		recipient_id,
		token_id,
		exercise_id,
		amount,
	)
	.await
	.map_err(|e| Error::Chain(e.to_string()))
}
