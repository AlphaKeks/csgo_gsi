#[cfg(feature = "gokz")]
use gokz_rs::SteamID;

use serde::{Deserialize, Serialize};

/// Information about a CS:GO game instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameInfo {
	/// The name of the game being played. (CS:GO)
	pub name: String,

	/// The ID of the game being played. (730)
	pub app_id: u32,

	/// The version of the game being played.
	pub version: u32,

	#[cfg(not(feature = "gokz"))]
	#[serde(rename = "steamid")]
	/// The player's `SteamID`
	pub steam_id: String,

	#[cfg(feature = "gokz")]
	#[serde(rename = "steamid")]
	/// The player's [`SteamID`]
	pub steam_id: SteamID,

	/// The timestamp of when the update happened.
	pub timestamp: u64,
}
