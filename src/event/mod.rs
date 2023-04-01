use {
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

mod game_info;
pub use game_info::GameInfo;

/// Module containing types for `Map` events.
pub mod map;
use map::Map;

/// Module containing types for `Player` events.
pub mod player;
use player::Player;

/// Module containing types for `Round` events.
pub mod round;
use round::Round;

/// The two teams in CS:GO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Team {
	/// Terrorists
	T,

	/// Counter-Terrorists
	CT,
}

/// An event received from CS:GO
/// {
///     "auth": Object {},
///     "map": Object {
///         "current_spectators": Number(0),
///         "mode": String("casual"),
///         "name": String("kz_xmas2008"),
///         "num_matches_to_win_series": Number(0),
///         "phase": String("warmup"),
///         "round": Number(0),
///         "souvenirs_total": Number(0),
///         "team_ct": Object {
///             "consecutive_round_losses": Number(0),
///             "matches_won_this_series": Number(0),
///             "score": Number(0),
///             "timeouts_remaining": Number(1),
///         },
///         "team_t": Object {
///             "consecutive_round_losses": Number(0),
///             "matches_won_this_series": Number(0),
///             "score": Number(0),
///             "timeouts_remaining": Number(1),
///         },
///     },
///     "player": Object {
///         "activity": String("textinput"),
///         "clan": String("[SKZ Beginner]"),
///         "name": String("dawn"),
///         "observer_slot": Number(1),
///         "steamid": String("76561199489033516"),
///         "team": String("CT"),
///     },
/// }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
	/// The current CS:GO Instance
	#[serde(rename = "provider")]
	pub game_info: Option<GameInfo>,

	/// The map currently being played.
	pub map: Option<Map>,

	/// The player who is currently playing / being spectated.
	pub player: Option<Player>,

	/// The current ingame round.
	pub round: Option<Round>,

	/// Authentication information.
	#[serde(default)]
	pub auth: HashMap<String, String>,
}
