use starknet::{
	accounts::SingleOwnerAccount,
	core::{chain_id, types::FieldElement},
	providers::SequencerGatewayProvider,
	signers::{LocalWallet, SigningKey},
};

lazy_static! {
	pub(super) static ref PROFILE_REGISTRY_CONTRACT_ADDRESS: FieldElement =
		FieldElement::from_hex_be(env!("PROFILE_REGISTRY_CONTRACT_ADDRESS")).unwrap();
	pub(super) static ref NFT_CONTRACT_ADDRESS: FieldElement =
		FieldElement::from_hex_be(env!("NFT_CONTRACT_ADDRESS")).unwrap();
	pub(super) static ref MINTER_ACCOUNT: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> = {
		let signer = LocalWallet::from(SigningKey::from_secret_scalar(
			FieldElement::from_hex_be(env!("MINTER_PRIVATE_KEY")).unwrap(),
		));
		let address = FieldElement::from_hex_be(env!("MINTER_ACCOUNT_CONTRACT_ADDRESS")).unwrap();

		SingleOwnerAccount::new(get_provider(), signer, address, get_chain_id())
	};
}

#[cfg(test)]
pub(crate) fn get_provider() -> SequencerGatewayProvider {
	SequencerGatewayProvider::starknet_nile_localhost()
}

#[cfg(not(test))]
pub(crate) fn get_provider() -> SequencerGatewayProvider {
	SequencerGatewayProvider::starknet_alpha_goerli()
}

fn get_chain_id() -> FieldElement {
	chain_id::TESTNET
}
