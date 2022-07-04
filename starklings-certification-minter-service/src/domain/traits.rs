use super::errors::Error;
use async_trait::async_trait;
use starknet::core::types::FieldElement;

pub type GithubUserId = u64;

#[async_trait]
pub(crate) trait GithubUserGetter {
	/// Return the GithubUserId associated with a github authentication token
	async fn get_user_id(&self, token: &str) -> Result<GithubUserId, Error>;
}

pub(crate) trait CourseConfigGetter {
	/// Return the NFT id associated to a Starkling section
	fn get_nft_id_for_section(&self, section_name: &str) -> Option<FieldElement>;
	/// Return the unique identifier associated to a Starkling exercise
	fn get_id_and_points_for_exercise(
		&self,
		exercise: &str,
	) -> Option<(FieldElement, FieldElement)>;
}
