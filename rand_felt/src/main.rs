use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet::core::types::FieldElement;

fn main() {
	let mut buffer = [0u8; 32];

	let mut rng = StdRng::from_entropy();
	rng.fill_bytes(&mut buffer[1..]);

	let random_felt = FieldElement::from_bytes_be(&buffer).unwrap();
	println!("{:#x}", random_felt);
}
