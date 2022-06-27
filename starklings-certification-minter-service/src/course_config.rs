use serde::Deserialize;
use starknet::core::types::FieldElement;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Deserialize)]
struct Exercise {
	id: String,
	name: String,
	points: u32,
}

#[derive(Deserialize)]
struct Section {
	id: u32,
	name: String,
	exercises: Vec<Exercise>,
}

#[derive(Deserialize)]
struct Config {
	sections: Vec<Section>,
}

lazy_static! {
	static ref COURSE_CONFIG: (HashMap<String, u32>, HashMap<String, (FieldElement, u32)>) = {
		let mut sections = HashMap::new();
		let mut exercises = HashMap::new();

		let course_config = fs::read_to_string(PathBuf::from("course.toml")).unwrap();
		let config: Config = toml::from_str(&course_config).unwrap();

		for section in config.sections {
			for exercise in section.exercises {
				exercises.insert(
					format!("{}/{}", section.name, exercise.name),
					(
						FieldElement::from_hex_be(&exercise.id).unwrap(),
						exercise.points,
					),
				);
			}
			sections.insert(section.name, section.id);
		}

		(sections, exercises)
	};
}

/// Return the NFT id associated to a Starkling section
pub(super) fn get_nft_id_for_section(section_name: &str) -> Option<FieldElement> {
	COURSE_CONFIG.0.get(section_name).map(|&v| FieldElement::from(v))
}

/// Return the unique identifier associated to a Starkling exercise
///
/// `exercise` must follow the "<section_name>/<exercise_name>" format
pub(super) fn get_uui_and_points_for_exercise(
	exercise: &str,
) -> Option<(FieldElement, FieldElement)> {
	COURSE_CONFIG.1.get(exercise).map(|(id, pts)| (*id, FieldElement::from(*pts)))
}
