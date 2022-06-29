pub mod only_dust_config;
mod profile_id;

pub use profile_id::ProfileId;
use starknet::{
	accounts::{Account, Call},
	core::types::{
		AddTransactionResult, BlockId, CallContractResult, FieldElement,
		InvokeFunctionTransactionRequest,
	},
	macros::selector,
	providers::Provider,
};

pub async fn mint_nft<A: Account + Sync>(
	minter_account: A,
	nft_contract_address: FieldElement,
	recipient_id: ProfileId,
	token_id: FieldElement,
	exercise_id: FieldElement,
	amount: FieldElement,
) -> Result<AddTransactionResult, A::SendTransactionError> {
	// Build the call payload
	let mut calldata = Vec::with_capacity(5);
	calldata.push(recipient_id.low());
	calldata.push(recipient_id.high());
	calldata.push(token_id);
	calldata.push(exercise_id);
	calldata.push(amount);

	// Create the transaction
	minter_account
		.execute(&[Call {
			to: nft_contract_address,
			selector: selector!("mint"),
			calldata,
		}])
		.send()
		.await
}

pub async fn get_profile_id<P: Provider>(
	provider: P,
	profile_registry_contract_address: FieldElement,
	github_id: FieldElement,
) -> Result<CallContractResult, P::Error> {
	let calldata = vec![github_id];

	provider
		.call_contract(
			InvokeFunctionTransactionRequest {
				contract_address: profile_registry_contract_address,
				entry_point_selector: selector!("get_user_information_from_github_handle"),
				calldata,
				signature: Default::default(), // Signing the call is not required
				max_fee: Default::default(),   // It is a view call, no fee will be paid
			},
			BlockId::Latest,
		)
		.await
}
