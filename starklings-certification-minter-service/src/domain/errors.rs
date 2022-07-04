use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
	InvalidExercisePath(String),
	UnknownSection(String),
	UnknownExercise(String),
	Github(String),
	Chain(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let output: &str = match self {
			Error::InvalidExercisePath(s) => s,
			Error::UnknownSection(s) => s,
			Error::UnknownExercise(s) => s,
			Error::Github(s) => s,
			Error::Chain(s) => s,
		};

		write!(f, "{}", output)
	}
}
