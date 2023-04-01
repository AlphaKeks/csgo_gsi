use {
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

/// Information about a CS:GO game instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
	/// The name of the map.
	pub name: String,

	/// The gamemode that is currently being played.
	pub mode: GameMode,

	/// The current phase of the game.
	pub phase: Phase,

	/// The current round number.
	pub round: u8,

	/// The amount of current spectators.
	#[serde(rename = "current_spectators")]
	pub spectator_count: usize,

	/// Who won which round and how?
	#[serde(default)]
	pub round_wins: HashMap<u64, String>,

	/// The number of matches to win series.
	#[serde(rename = "num_matches_to_win_series")]
	pub matches_to_win: usize,

	/// The amount of souvenirs dropped on this map so far.
	pub souvenirs_total: usize,

	/// The Terrorist's current stats.
	#[serde(rename = "team_ct")]
	pub t_stats: Stats,

	/// The Counter-Terrorist's current stats.
	#[serde(rename = "team_t")]
	pub ct_stats: Stats,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
	Competetive,
	Casual,
	Deathmatch,
	Training,
	#[serde(rename = "gungametrbomb")]
	Demolition,
	#[serde(rename = "gungameprogressive")]
	ArmsRace,
	#[serde(rename = "scrimcomp2v2")]
	Wingman,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
	Warmup,
	Live,
	Halftime,
	GameOver,
}

/// The stats of the current game.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Stats {
	/// Team name.
	pub name: Option<String>,

	/// Flag code.
	pub flag: Option<String>,

	/// The amount of won rounds.
	pub score: usize,

	/// The amount of lost rounds in a row.
	#[serde(rename = "consecutive_round_losses")]
	pub loss_streak: usize,

	/// The amount of timeouts remaining.
	#[serde(rename = "timeouts_remaining")]
	pub timeouts: usize,

	/// The amount of wins this series.
	#[serde(rename = "matches_won_this_series")]
	pub wins: usize,
}
