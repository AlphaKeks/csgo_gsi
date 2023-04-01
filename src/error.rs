use {
	serde::{Deserialize, Serialize},
	std::fmt::Display,
	tracing::error,
};

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-wide error type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Error {
	/// Failed to serialize config as VDF.
	SerializeVdf,
	/// Failed to write config file to disk.
	SaveConfig,
	#[cfg(unix)]
	/// Could not find home directory on UNIX system.
	NoHomeDir,
	/// Could not find CS:GO cfg directory.
	NoCfgDir,
	/// Failed to start Axum HTTP server
	Axum,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Error::SerializeVdf => "Failed to serialize config as VDF.",
			Error::SaveConfig => "Failed to write config file to disk.",
			#[cfg(unix)]
			Error::NoHomeDir => "Could not find $HOME directory.",
			Error::NoCfgDir => "Could not find /csgo/cfg directory.",
			Error::Axum => "Failed to run Axum server.",
		})
	}
}

impl std::error::Error for Error {}

impl From<vdf_serde::Error> for Error {
	fn from(value: vdf_serde::Error) -> Self {
		error!("{value:?}");
		Self::SerializeVdf
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		error!("{value:?}");
		Self::SaveConfig
	}
}

#[cfg(unix)]
impl From<std::env::VarError> for Error {
	fn from(value: std::env::VarError) -> Self {
		error!("{value:?}");
		Self::NoHomeDir
	}
}
