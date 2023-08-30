//! Module containing all the CS:GO specific types.

use std::collections::HashMap;

/// An event emitted by CS:GO.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Event {
	/// The current CS:GO instance.
	pub provider: Option<GameInfo>,

	/// Auth state.
	#[cfg_attr(feature = "serde", serde(default))]
	pub auth: HashMap<String, String>,

	/// The current player.
	///
	/// If the game is spectating a player, that player will be used.
	pub player: Option<Player>,

	/// The current map.
	///
	/// Is [`None`] while in the main menu.
	pub map: Option<Map>,

	/// The current round of the game.
	///
	/// Is [`None`] while in the main menu.
	pub round: Option<Round>,
}

/// Information about a CS:GO instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameInfo {
	/// The name of the game.
	pub name: String,

	/// The ID of the game. `730` in the case of CS:GO.
	pub app_id: u16,

	/// The version of the game.
	pub version: u32,

	/// The `SteamID` of the player playing the game.
	#[cfg(not(feature = "gokz"))]
	#[cfg_attr(feature = "serde", serde(rename = "steamid"))]
	pub steam_id: String,

	/// The `SteamID` of the player playing the game.
	#[cfg(feature = "gokz")]
	#[cfg_attr(feature = "serde", serde(rename = "steamid"))]
	pub steam_id: gokz_rs::SteamID,

	/// The timestamp of when this update was generated.
	pub timestamp: u64,
}

/// Information about a CS:GO instance.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Player {
	/// The player's name.
	pub name: String,

	/// The player's `SteamID`.
	#[cfg(not(feature = "gokz"))]
	#[cfg_attr(feature = "serde", serde(rename = "steamid"))]
	pub steam_id: String,

	/// The player's [`SteamID`](gokz_rs::SteamID).
	#[cfg(feature = "gokz")]
	#[cfg_attr(feature = "serde", serde(rename = "steamid"))]
	pub steam_id: gokz_rs::SteamID,

	/// What the player is currently doing.
	pub activity: PlayerActivity,

	/// The player's clan tag, if they have one selected.
	pub clan: Option<String>,

	/// The player's team.
	pub team: Option<Team>,

	/// The player's weapons.
	#[cfg_attr(feature = "serde", serde(default))]
	pub weapons: HashMap<String, Weapon>,

	/// The player's current game state.
	pub state: Option<PlayerState>,

	/// The player's stats for this match.
	pub match_stats: Option<MatchStats>,

	/// The player's spectator slot.
	pub observer_slot: Option<usize>,
}

/// A player's current activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PlayerActivity {
	/// The main menu.
	Menu,

	/// In-game.
	Playing,

	/// Chat opened.
	TextInput,
}

/// The two teams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Team {
	/// Terrorists.
	T,

	/// Counter-Terrorists.
	CT,
}

/// Information about a weapon.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Weapon {
	/// The name of the weapon.
	pub name: String,

	/// The name of the skin of the weapon.
	#[cfg_attr(feature = "serde", serde(rename = "paintkit"))]
	pub skin: String,

	/// The type of the weapon.
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub weapon_type: WeaponType,

	/// The state of the weapon.
	#[cfg_attr(feature = "serde", serde(rename = "state"))]
	pub weapon_state: WeaponState,

	/// The amount of ammo left.
	pub ammo_clip: Option<usize>,

	/// The maximum amount of ammo this weapon can hold.
	#[cfg_attr(feature = "serde", serde(rename = "max_ammo"))]
	pub ammo_max: Option<usize>,

	/// The amount of ammo left in reserve.
	pub ammo_reserve: Option<usize>,
}

/// The different types of weapons.
#[allow(missing_docs, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// The 3 states a weapon can be in.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WeaponState {
	Holstered,
	Active,
	Reloading,
}

/// The state of a player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerState {
	/// The player's health.
	pub health: u8,

	/// The player's armor.
	pub armor: u8,

	/// Whether the player is wearing a helmet.
	pub helmet: bool,

	/// How long the player will stay flashed for.
	pub flash_duration: usize,

	#[allow(missing_docs)] // No idea what this is.
	pub smoke_duration: usize,

	/// How long the player will stay on fire for.
	pub burn_duration: usize,

	/// How much money the player currently has.
	pub money: usize,

	/// How many kills the player currently has.
	///
	/// Can be negative if the player has more deaths / team kills than actual kills.
	pub kills: isize,

	/// How many headshots the player landed this game.
	pub headshots: usize,

	#[allow(missing_docs)] // No idea what this is.
	pub equipment_value: usize,

	/// How much damage the player has dealt this round.
	pub round_damage: Option<usize>,

	/// Whether the player is holding a defuse kit.
	#[cfg_attr(feature = "serde", serde(rename = "defusekit"))]
	pub defuse_kit: Option<bool>,
}

/// Stats for the current match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchStats {
	/// How many kills the player has made this match.
	///
	/// Can be negative if the player has more deaths / team kills than actual kills.
	pub kills: isize,

	/// How many kills the player has assisted with this match.
	pub assists: usize,

	/// How many times the player has died during this match.
	pub deaths: usize,

	/// How many times the player has been MVP this match.
	pub mvps: usize,

	/// How much score the player has achieved this match.
	pub score: usize,
}

/// Information about a CS:GO instance.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
	/// The name of the map.
	pub name: String,

	/// The game mode being played.
	pub mode: GameMode,

	/// The current phase of the game.
	pub phase: GamePhase,

	/// The current round being played.
	pub round: u8,

	/// The amount of current spectators.
	#[cfg_attr(feature = "serde", serde(rename = "current_spectators"))]
	pub spectator_count: usize,

	/// Who won which round and how often.
	#[cfg_attr(feature = "serde", serde(default))]
	pub round_wins: HashMap<u64, String>,

	/// How many matches are required until a team wins.
	#[cfg_attr(feature = "serde", serde(rename = "num_matches_to_win_series"))]
	pub matches_to_win: usize,

	/// The amount of souvenier drops on this map.
	pub souveniers_total: usize,

	/// The Terrorist's current stats.
	#[cfg_attr(feature = "serde", serde(rename = "team_t"))]
	pub t_stats: GameStats,

	/// The Counter-Terrorist's current stats.
	#[cfg_attr(feature = "serde", serde(rename = "team_ct"))]
	pub ct_stats: GameStats,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameMode {
	Competetive,
	Casual,
	Deathmatch,
	Training,
	#[cfg_attr(feature = "serde", serde(rename = "gungametrbomb"))]
	Demolition,
	#[cfg_attr(feature = "serde", serde(rename = "gungameprogressive"))]
	ArmsRace,
	#[cfg_attr(feature = "serde", serde(rename = "scrimcomp2v2"))]
	Wingman,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum GamePhase {
	Warmup,
	Live,
	Halftime,
	GameOver,
}

/// Stats about the current game.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameStats {
	/// Team name.
	pub name: Option<String>,

	#[allow(missing_docs)] // No idea what this is.
	pub flag: Option<String>,

	/// The amount of won rounds.
	pub score: usize,

	/// The amount of lost rounds in a row.
	#[cfg_attr(feature = "serde", serde(rename = "consecutive_round_losses"))]
	pub loss_streak: usize,

	/// The amount of timeouts remaining.
	#[cfg_attr(feature = "serde", serde(rename = "timeouts_remaining"))]
	pub timeouts: usize,

	/// The amount of wins this series.
	#[cfg_attr(feature = "serde", serde(rename = "matches_won_this_series"))]
	pub wins: usize,
}

/// Information about the current game round.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Round {
	/// The current phase of the round.
	pub phase: RoundPhase,

	/// The state of the bomb.
	pub bomb_state: Option<BombState>,

	/// The winner of this round.
	pub winner: Option<Team>,
}

/// The phase of a round.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RoundPhase {
	FreezeTime,
	Live,
	Over,
}

/// The state of the bomb.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BombState {
	Planted,
	Defused,
	Exploded,
}
