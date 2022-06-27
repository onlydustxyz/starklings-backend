use starknet::core::types::FieldElement;

/// The OnlydDust unique identifier for contributors
pub struct ProfileId {
	low: FieldElement,
	high: FieldElement,
}

impl ProfileId {
	pub fn new(low: FieldElement, high: FieldElement) -> Self {
		ProfileId { low, high }
	}

	pub fn low(&self) -> FieldElement {
		self.low
	}

	pub fn high(&self) -> FieldElement {
		self.high
	}
}

impl TryFrom<Vec<FieldElement>> for ProfileId {
	type Error = &'static str;

	fn try_from(value: Vec<FieldElement>) -> Result<Self, Self::Error> {
		Ok(ProfileId::new(
			*value.get(0).ok_or("ProfileId require two FieldElement")?,
			*value.get(1).ok_or("ProfileId require two FieldElement")?,
		))
	}
}
