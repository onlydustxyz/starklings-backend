use only_dust_contracts_api::only_dust_config::{OnlyDustConfig, OnlyDustConfigGetter};
use starknet::{accounts::Account, core::types::FieldElement};

use crate::{
	course_config::{CourseConfig, CourseConfigGetter},
	ReadOnlySharedState,
};

impl<A: Account + Clone> OnlyDustConfigGetter<A> for ReadOnlySharedState<OnlyDustConfig<A>> {
	fn profile_registry_contract_address(&self) -> FieldElement {
		let config_lock = self.read().unwrap();
		config_lock.profile_registry_contract_address()
	}

	fn nft_contract_address(&self) -> FieldElement {
		let config_lock = self.read().unwrap();
		config_lock.nft_contract_address()
	}

	fn minter_account(&self) -> A {
		let config_lock = self.read().unwrap();
		config_lock.minter_account()
	}
}

impl CourseConfigGetter for ReadOnlySharedState<CourseConfig> {
	fn get_nft_id_for_section(&self, section_name: &str) -> Option<FieldElement> {
		let config_lock = self.read().unwrap();
		config_lock.get_nft_id_for_section(section_name)
	}

	fn get_id_and_points_for_exercise(
		&self,
		exercise: &str,
	) -> Option<(FieldElement, FieldElement)> {
		let config_lock = self.read().unwrap();
		config_lock.get_id_and_points_for_exercise(exercise)
	}
}
