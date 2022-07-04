use starknet::core::types::FieldElement;

use super::{errors::Error, types::ProfileId};
use async_trait::async_trait;

#[async_trait]
pub(crate) trait OnlyDustCalls {
	async fn mint_nft(
		&self,
		recipient_id: ProfileId,
		token_id: FieldElement,
		exercise_id: FieldElement,
		amount: FieldElement,
	) -> Result<FieldElement, Error>;

	async fn get_profile_id(&self, github_id: FieldElement) -> Result<ProfileId, Error>;
}
