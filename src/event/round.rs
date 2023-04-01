use serde::{Deserialize, Serialize};

/// Information about the current Round.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Round {
	pub phase: Phase,
	pub bomb_state: Option<BombState>,
	pub winner: Option<super::Team>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
	FreezeTime,
	Live,
	Over,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
	Planted,
	Defused,
	Exploded,
}
