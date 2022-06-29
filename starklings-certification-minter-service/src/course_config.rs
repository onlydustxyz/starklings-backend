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
struct Toml {
	sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub struct CourseConfig {
	sections: HashMap<String, u32>,
	exercises: HashMap<String, (FieldElement, u32)>,
}

impl CourseConfig {
	pub fn read_from_toml(path: Option<PathBuf>) -> Self {
		let mut sections = HashMap::new();
		let mut exercises = HashMap::new();

		let toml_path: PathBuf = path.unwrap_or_else(|| PathBuf::from("course.toml"));

		let course_config = fs::read_to_string(toml_path).unwrap();
		let config: Toml = toml::from_str(&course_config).unwrap();

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

		Self {
			sections,
			exercises,
		}
	}
}

pub trait CourseConfigGetter {
	/// Return the NFT id associated to a Starkling section
	fn get_nft_id_for_section(&self, section_name: &str) -> Option<FieldElement>;
	/// Return the unique identifier associated to a Starkling exercise
	fn get_id_and_points_for_exercise(
		&self,
		exercise: &str,
	) -> Option<(FieldElement, FieldElement)>;
}

impl CourseConfigGetter for CourseConfig {
	fn get_nft_id_for_section(&self, section_name: &str) -> Option<FieldElement> {
		self.sections.get(section_name).map(|&v| FieldElement::from(v))
	}

	fn get_id_and_points_for_exercise(
		&self,
		exercise: &str,
	) -> Option<(FieldElement, FieldElement)> {
		self.exercises.get(exercise).map(|(id, pts)| (*id, FieldElement::from(*pts)))
	}
}
