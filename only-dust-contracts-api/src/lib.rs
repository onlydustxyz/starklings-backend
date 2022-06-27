#[macro_use]
extern crate lazy_static;

mod config;
mod profile_id;

use config::{MINTER_ACCOUNT, NFT_CONTRACT_ADDRESS, PROFILE_REGISTRY_CONTRACT_ADDRESS};
pub use profile_id::ProfileId;
use starknet::{
	accounts::{Account, Call, SingleOwnerAccount},
	core::types::{
		AddTransactionResult, BlockId, CallContractResult, FieldElement,
		InvokeFunctionTransactionRequest,
	},
	macros::selector,
	providers::{Provider, SequencerGatewayProvider, SequencerGatewayProviderError},
	signers::LocalWallet,
};

pub async fn mint_nft(
	recipient_id: ProfileId,
	token_id: FieldElement,
	exercise_id: FieldElement,
	amount: FieldElement,
) -> Result<
	AddTransactionResult,
	<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> as Account>::SendTransactionError,
> {
	// Build the call payload
	let mut calldata = Vec::with_capacity(5);
	calldata.push(recipient_id.low());
	calldata.push(recipient_id.high());
	calldata.push(token_id);
	calldata.push(exercise_id);
	calldata.push(amount);

	// Create the transaction
	MINTER_ACCOUNT
		.execute(&[Call {
			to: *NFT_CONTRACT_ADDRESS,
			selector: selector!("mint"),
			calldata,
		}])
		.send()
		.await
}

pub async fn get_profile_id(
	github_id: FieldElement,
) -> Result<CallContractResult, SequencerGatewayProviderError> {
	let provider = config::get_provider();
	let calldata = vec![github_id];

	provider
		.call_contract(
			InvokeFunctionTransactionRequest {
				contract_address: *PROFILE_REGISTRY_CONTRACT_ADDRESS,
				entry_point_selector: selector!("get_user_information_from_github_handle"),
				calldata,
				signature: Default::default(), // Signing the call is not required
				max_fee: Default::default(),   // It is a view call, no fee will be paid
			},
			BlockId::Latest,
		)
		.await
}
