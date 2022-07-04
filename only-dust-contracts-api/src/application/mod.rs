use starknet::core::types::FieldElement;

use crate::{
	domain::{errors::Error, traits::OnlyDustCalls, types::ProfileId},
	infrastructure::onlydust_chain_config::OnlyDustChainConfig,
};

/// Increase a Profile level of certification
pub async fn increase_certification_score(
	recipient_id: ProfileId,
	certification_id: FieldElement,
	milestone_id: FieldElement,
	amount: FieldElement,
) -> Result<FieldElement, Error> {
	ONLYDUST_CONFIG
		.mint_nft(recipient_id, certification_id, milestone_id, amount)
		.await
}

/// Return the Profile associated to a Github user
pub async fn get_onlydust_profile_for_github_user(
	github_id: FieldElement,
) -> Result<ProfileId, Error> {
	ONLYDUST_CONFIG.get_profile_id(github_id).await
}

lazy_static! {
	static ref ONLYDUST_CONFIG: OnlyDustChainConfig = OnlyDustChainConfig::read_from_env();
}
