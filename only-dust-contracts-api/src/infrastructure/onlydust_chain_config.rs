use async_trait::async_trait;
use std::env;

use starknet::{
	accounts::{Account, Call, SingleOwnerAccount},
	core::{
		chain_id,
		types::{BlockId, FieldElement, InvokeFunctionTransactionRequest},
	},
	macros::selector,
	providers::{Provider, SequencerGatewayProvider},
	signers::{LocalWallet, SigningKey},
};

use crate::domain::{errors::Error, traits::OnlyDustCalls, types::ProfileId};

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

#[derive(Clone)]
pub(crate) struct OnlyDustChainConfig {
	profile_registry_contract_address: FieldElement,
	nft_contract_address: FieldElement,
	minter_account: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>,
}

impl OnlyDustChainConfig {
	pub fn profile_registry_contract_address(&self) -> FieldElement {
		self.profile_registry_contract_address
	}

	pub fn nft_contract_address(&self) -> FieldElement {
		self.nft_contract_address
	}

	pub fn minter_account(&self) -> SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> {
		self.minter_account.clone()
	}
}

impl OnlyDustChainConfig {
	pub fn read_from_env() -> Self {
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

		Self {
			profile_registry_contract_address,
			nft_contract_address,
			minter_account,
		}
	}
}

#[async_trait]
impl OnlyDustCalls for OnlyDustChainConfig {
	async fn mint_nft(
		&self,
		recipient_id: ProfileId,
		token_id: FieldElement,
		exercise_id: FieldElement,
		amount: FieldElement,
	) -> Result<FieldElement, Error> {
		let calldata = vec![
			recipient_id.low(),
			recipient_id.high(),
			token_id,
			exercise_id,
			amount,
		];

		// Create the transaction
		let response = self
			.minter_account()
			.execute(&[Call {
				to: self.nft_contract_address(),
				selector: selector!("mint"),
				calldata,
			}])
			.send()
			.await
			.map_err(|e| Error::SendTransaction(e.to_string()))?;

		Ok(response.transaction_hash)
	}

	async fn get_profile_id(
		&self,
		github_id: starknet::core::types::FieldElement,
	) -> Result<ProfileId, Error> {
		let calldata = vec![github_id];

		let provider = get_provider();

		let response = provider
			.call_contract(
				InvokeFunctionTransactionRequest {
					contract_address: self.profile_registry_contract_address(),
					entry_point_selector: selector!("get_user_information_from_github_handle"),
					calldata,
					signature: Default::default(), // Signing the call is not required
					max_fee: Default::default(),   // It is a view call, no fee will be paid
				},
				BlockId::Latest,
			)
			.await
			.map_err(|e| Error::CallContract(e.to_string()))?;

		let profile_id =
			ProfileId::try_from(response.result).map_err(|_| Error::InvalidValueReturned)?;

		Ok(profile_id)
	}
}
