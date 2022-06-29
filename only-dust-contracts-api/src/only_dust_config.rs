use std::env;

use starknet::{
	accounts::{Account, SingleOwnerAccount},
	core::{chain_id, types::FieldElement},
	providers::SequencerGatewayProvider,
	signers::{LocalWallet, SigningKey},
};

#[cfg(test)]
pub fn get_provider() -> SequencerGatewayProvider {
	SequencerGatewayProvider::starknet_nile_localhost()
}

#[cfg(not(test))]
pub fn get_provider() -> SequencerGatewayProvider {
	SequencerGatewayProvider::starknet_alpha_goerli()
}

fn get_chain_id() -> FieldElement {
	chain_id::TESTNET
}

#[derive(Debug, Clone)]
pub struct OnlyDustConfig<A: Account + Clone> {
	profile_registry_contract_address: FieldElement,
	nft_contract_address: FieldElement,
	minter_account: A,
}

pub trait OnlyDustConfigGetter<A: Account + Clone> {
	fn profile_registry_contract_address(&self) -> FieldElement;

	fn nft_contract_address(&self) -> FieldElement;

	fn minter_account(&self) -> A;
}

impl<A: Account + Clone> OnlyDustConfigGetter<A> for OnlyDustConfig<A> {
	fn profile_registry_contract_address(&self) -> FieldElement {
		self.profile_registry_contract_address
	}

	fn nft_contract_address(&self) -> FieldElement {
		self.nft_contract_address
	}

	fn minter_account(&self) -> A {
		self.minter_account.clone()
	}
}

impl OnlyDustConfig<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>> {
	pub fn read_from_env() -> (
		FieldElement,
		FieldElement,
		SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>,
	) {
		let profile_registry_contract_address =
			FieldElement::from_hex_be(&env::var("PROFILE_REGISTRY_CONTRACT_ADDRESS").unwrap())
				.unwrap();
		let nft_contract_address =
			FieldElement::from_hex_be(&env::var("NFT_CONTRACT_ADDRESS").unwrap()).unwrap();
		let minter_account: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> = {
			let signer = LocalWallet::from(SigningKey::from_secret_scalar(
				FieldElement::from_hex_be(&env::var("MINTER_PRIVATE_KEY").unwrap()).unwrap(),
			));
			let address =
				FieldElement::from_hex_be(&env::var("MINTER_ACCOUNT_CONTRACT_ADDRESS").unwrap())
					.unwrap();

			SingleOwnerAccount::new(get_provider(), signer, address, get_chain_id())
		};

		(
			profile_registry_contract_address,
			nft_contract_address,
			minter_account,
		)
	}
}
