use std::fmt::Display;

/// Interaction error with OnlyDust on-chain ressources
#[derive(Debug)]
pub enum Error {
	SendTransaction(String),
	CallContract(String),
	InvalidValueReturned,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let output: &str = match self {
			Error::SendTransaction(s) => s,
			Error::CallContract(s) => s,
			Error::InvalidValueReturned => "value found on chain does not match expectations",
		};

		write!(f, "{}", output)
	}
}
