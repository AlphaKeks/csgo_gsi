use {
	std::{io, result::Result as StdResult},
	thiserror::Error as ThisError,
};

/// Type alias for a [`Result`](StdResult) with [`Error`] as the `Err` variant.
pub type Result<T> = StdResult<T, Error>;

/// Crate-wide error type.
///
/// Any fallible function in this crate will return a [`Result`] with this as the `Err` variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, ThisError)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
	/// Something went wrong serializing the config file.
	#[error("Failed to serialize config file: {0}")]
	SerializeConfig(String),

	/// Something went wrong saving the config file.
	#[error("Failed to save config file to disk: {0}")]
	SaveConfig(String),
}

impl From<vdf_serde::Error> for Error {
	fn from(error: vdf_serde::Error) -> Self {
		Self::SerializeConfig(error.to_string())
	}
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Self::SaveConfig(error.to_string())
	}
}
