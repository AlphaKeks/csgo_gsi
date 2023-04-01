#[cfg(feature = "gokz")]
use gokz_rs::SteamID;

use {
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

/// Information about the current Player.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
	/// The name of the map.
	pub name: String,

	#[cfg(not(feature = "gokz"))]
	#[serde(rename = "steamid")]
	/// The player's `SteamID`
	pub steam_id: String,

	#[cfg(feature = "gokz")]
	#[serde(rename = "steamid")]
	/// The player's [`SteamID`]
	pub steam_id: SteamID,

	/// The player's current activity.
	pub activity: Activity,

	/// The player's clan tag.
	pub clan: Option<String>,

	/// The team the player is currently on.
	pub team: Option<super::Team>,

	/// The weapons that the player is currently holding.
	#[serde(default)]
	pub weapons: HashMap<String, Weapon>,

	/// The player's current state (health, armor, etc.).
	pub state: Option<State>,

	/// The player's stats for the current round.
	pub match_stats: Option<MatchStats>,

	/// The player's observer slot number.
	pub observer_slot: Option<usize>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
	Menu,
	Playing,
	TextInput,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Weapon {
	pub name: String,
	#[serde(rename = "paintkit")]
	pub skin: String,
	#[serde(rename = "type")]
	pub weapon_type: WeaponType,
	pub state: WeaponState,
	pub ammo_clip: Option<usize>,
	pub max_ammo: Option<usize>,
	pub ammo_reserve: Option<usize>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WeaponType {
	Knife,
	Pistol,
	SMG,
	MachineGun,
	Rifle,
	SniperRifle,
	Shotgun,
	StackableItem,
	Grenage,
	C4,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WeaponState {
	Holstered,
	Active,
	Reloading,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct State {
	pub health: u8,
	pub armor: u8,
	pub helmet: bool,
	pub flash_duration: usize,
	pub smoke_duration: usize,
	pub burn_duration: usize,
	pub money: usize,
	pub kills: isize,
	pub headshots: usize,
	pub equipment_value: usize,
	pub round_damage: Option<usize>,
	#[serde(rename = "defusekit")]
	pub defuse_kit: Option<bool>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MatchStats {
	pub kills: isize,
	pub assists: usize,
	pub deaths: usize,
	pub mvps: usize,
	pub score: usize,
}
