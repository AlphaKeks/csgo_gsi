//! Utility module to filter out unnecessary information from [`crate::Event`]s and parse types
//! into [`gokz_rs`] types.

use {
	crate::{Error, Result},
	gokz_rs::{Mode, SteamID},
	serde::{Deserialize, Serialize},
};

const KZ_MAP_PREFIXES: [&str; 6] = ["kz_", "kzpro_", "skz_", "vnl_", "bkz_", "xc_"];

/// An event emitted by CS:GO, with all the KZ relevant information.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event {
	/// The [`SteamID`] of the current player.
	pub steam_id: SteamID,

	/// The name of the current player.
	pub player_name: String,

	/// The name of the current map.
	pub map_name: Option<String>,

	/// The name of the [`Mode`] the current player is in.
	pub mode: Option<Mode>,
}

impl TryFrom<crate::Event> for Event {
	type Error = Error;

	fn try_from(event: crate::Event) -> Result<Self> {
		let Some(player) = event.player else {
			return Err(Error::NoPlayer);
		};

		let map_name = event.map.and_then(|map| {
			let map_name = if map.name.contains('/') {
				let (_, name) = map.name.rsplit_once('/')?;
				name.to_owned()
			} else {
				map.name
			};

			KZ_MAP_PREFIXES
				.iter()
				.any(|prefix| map_name.starts_with(prefix))
				.then_some(map_name)
		});

		let mode = player
			.clan
			.map(|clan_tag| clan_tag.replace(['[', ']'], ""))
			.and_then(|clan_tag| clan_tag.split_whitespace().next()?.parse().ok());

		Ok(Self { steam_id: player.steam_id, player_name: player.name, map_name, mode })
	}
}
