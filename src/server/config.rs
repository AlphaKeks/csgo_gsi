use {
	crate::Result,
	serde::{Deserialize, Serialize},
	std::{
		collections::{HashMap, HashSet},
		path::PathBuf,
		time::Duration,
	},
	tracing::debug,
};

/// Configuration for CS:GO's GSI feature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
	/// The name of your service.
	pub service_name: String,

	/// The port of your service.
	pub service_port: u16,

	/// The time CS:GO will wait for a response until it considers an event as
	/// "received".
	pub timeout: Duration,

	/// The game will wait this much time collecting events before sending them.
	///
	/// Can be set to 0 in localhost games but should be positive for online games.
	pub buffer: Duration,

	/// CS:GO will wait at least this duration between sending events.
	pub throttle: Duration,

	/// CS:GO will wait this duration between sending game updates even though nothing
	/// has changed.
	pub heartbeat: Duration,

	#[allow(missing_docs)]
	pub auth: HashMap<String, String>,

	/// How many digits after decimal point get reported.
	pub precision_time: u8,

	/// How many digits after decimal point get reported.
	pub precision_position: u8,

	/// How many digits after decimal point get reported.
	pub precision_vector: u8,

	/// The events you want to subscribe to.
	pub subscriptions: HashSet<Subscription>,
}

impl Config {
	/// The default buffer size.
	pub const DEFAULT_BUFFER: Duration = Duration::from_millis(100);
	/// The default hearbeat duration.
	pub const DEFAULT_HEARTBEAT: Duration = Duration::from_secs(60);
	/// The default precision for floating point values.
	pub const DEFAULT_PRECISION: u8 = 3;
	/// The default throttle time.
	pub const DEFAULT_THROTTLE: Duration = Duration::from_secs(1);
	/// The default timeout.
	pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

	/// Constructs a default builder for [`Config`].
	pub fn builder() -> ConfigBuilder {
		ConfigBuilder::default()
	}

	/// Install the config as a file into the given `path`.
	///
	/// This `path` should be the `cfg` folder of your CS:GO installation.
	/// The default path for this, starting at your Steam Library root is
	///
	/// ```not_rust
	/// SteamLibrary/steamapps/common/Counter-Strike Global Offensive/csgo/cfg
	/// ```
	pub fn install(&self, path: impl Into<PathBuf>) -> Result<ConfigFile> {
		let mut path = path.into();
		let filename = format!("gamestate_integration_{}.cfg", self.service_name);

		debug!(path = %path.display(), %filename, "Installing config...");
		path.push(filename);

		let uri = format!("http://127.0.0.1:{}", self.service_port);

		let output = Precision {
			precision_time: self.precision_time,
			precision_position: self.precision_position,
			precision_vector: self.precision_vector,
		};

		let data = Data {
			map_round_wins: self
				.subscriptions
				.contains(&Subscription::MapRoundWins),
			map: self.subscriptions.contains(&Subscription::Map),
			player_id: self
				.subscriptions
				.contains(&Subscription::PlayerID),
			player_match_stats: self
				.subscriptions
				.contains(&Subscription::PlayerMatchStats),
			player_state: self
				.subscriptions
				.contains(&Subscription::PlayerState),
			player_weapons: self
				.subscriptions
				.contains(&Subscription::PlayerWeapons),
			provider: self
				.subscriptions
				.contains(&Subscription::Provider),
			round: self.subscriptions.contains(&Subscription::Round),
			allgrenades: self
				.subscriptions
				.contains(&Subscription::AllGrenades),
			allplayers_id: self
				.subscriptions
				.contains(&Subscription::AllPlayersID),
			allplayers_match_stats: self
				.subscriptions
				.contains(&Subscription::AllPlayersMatchStats),
			allplayers_position: self
				.subscriptions
				.contains(&Subscription::AllPlayersPosition),
			allplayers_state: self
				.subscriptions
				.contains(&Subscription::AllPlayersState),
			allplayers_weapons: self
				.subscriptions
				.contains(&Subscription::AllPlayersWeapons),
			bomb: self.subscriptions.contains(&Subscription::Bomb),
			phase_countdowns: self
				.subscriptions
				.contains(&Subscription::PhaseCountdowns),
			player_position: self
				.subscriptions
				.contains(&Subscription::PlayerPosition),
		};

		let config_file = ConfigFile {
			uri,
			timeout: self.timeout.as_secs_f64(),
			buffer: self.buffer.as_secs_f64(),
			throttle: self.throttle.as_secs_f64(),
			heartbeat: self.heartbeat.as_secs_f64(),
			auth: self.auth.clone(),
			output,
			data,
		};

		let config_vdf = vdf_serde::to_string(&config_file)?;

		std::fs::write(&path, config_vdf.as_bytes())?;
		debug!(path = %path.display(), "Wrote config to disk.");

		Ok(config_file)
	}
}

/// Events to subscribe to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subscription {
	/// History of round wins.
	MapRoundWins,
	/// Mode, map, phase, team scores
	Map,
	/// SteamID
	PlayerID,
	/// Scoreboard info
	PlayerMatchStats,
	/// armor, flashed, equip_value, health, etc.
	PlayerState,
	/// List of player weapons and weapon state.
	PlayerWeapons,
	/// Info about the game providing info.
	Provider,
	/// Round phase and the winning team.
	Round,
	/// Grenade effecttime, lifetime, owner, position, type, velocity.
	AllGrenades,
	/// The SteamID of each player.
	AllPlayersID,
	/// The scoreboard info for each player.
	AllPlayersMatchStats,
	/// Player_position but for each player.
	AllPlayersPosition,
	/// The player_state for each player.
	AllPlayersState,
	/// The player_weapons for each player.
	AllPlayersWeapons,
	/// Location of the bomb, who's carrying it, dropped or not.
	Bomb,
	/// Time remaining in tenths of a second, which phase.
	PhaseCountdowns,
	/// Forward direction, position for currently spectated player.
	PlayerPosition,
}

impl Subscription {
	/// All subscriptions that are available in any context.
	pub const UNRESTRICTED: &'static [Self] = &[
		Self::MapRoundWins,
		Self::Map,
		Self::PlayerID,
		Self::PlayerMatchStats,
		Self::PlayerState,
		Self::PlayerWeapons,
		Self::Provider,
		Self::Round,
	];
}

/// Builder for [`Config`].
#[derive(Default, Debug, Clone)]
pub struct ConfigBuilder {
	timeout: Option<Duration>,
	buffer: Option<Duration>,
	throttle: Option<Duration>,
	heartbeat: Option<Duration>,
	auth: Option<HashMap<String, String>>,
	precision_time: Option<u8>,
	precision_position: Option<u8>,
	precision_vector: Option<u8>,
	subscriptions: Option<HashSet<Subscription>>,
}

#[allow(missing_docs)]
impl ConfigBuilder {
	pub fn timeout(mut self, timeout: impl Into<Duration>) -> Self {
		self.timeout = Some(timeout.into());
		self
	}

	pub fn buffer(mut self, buffer: impl Into<Duration>) -> Self {
		self.buffer = Some(buffer.into());
		self
	}

	pub fn throttle(mut self, throttle: impl Into<Duration>) -> Self {
		self.throttle = Some(throttle.into());
		self
	}

	pub fn heartbeat(mut self, heartbeat: impl Into<Duration>) -> Self {
		self.heartbeat = Some(heartbeat.into());
		self
	}

	pub fn auth<K, V>(mut self, key: K, value: V) -> Self
	where
		K: Into<String>,
		V: Into<String>, {
		match self.auth.as_mut() {
			None => {
				let mut auth = HashMap::new();
				auth.insert(key.into(), value.into());
				self.auth = Some(auth);
			}
			Some(auth) => {
				auth.insert(key.into(), value.into());
			}
		};

		self
	}

	pub fn precision_time(mut self, precision_time: impl Into<u8>) -> Self {
		self.precision_time = Some(precision_time.into());
		self
	}

	pub fn precision_position(mut self, precision_position: impl Into<u8>) -> Self {
		self.precision_position = Some(precision_position.into());
		self
	}

	pub fn precision_vector(mut self, precision_vector: impl Into<u8>) -> Self {
		self.precision_vector = Some(precision_vector.into());
		self
	}

	pub fn subscribe(mut self, subscription: impl Into<Subscription>) -> Self {
		match self.subscriptions.as_mut() {
			None => {
				let mut subscriptions = HashSet::new();
				subscriptions.insert(subscription.into());
				self.subscriptions = Some(subscriptions);
			}
			Some(subscriptions) => {
				subscriptions.insert(subscription.into());
			}
		};

		self
	}

	pub fn subscribe_multiple<I, S>(mut self, iter: I) -> Self
	where
		I: Iterator<Item = S>,
		S: Into<Subscription>, {
		match self.subscriptions.as_mut() {
			None => {
				self.subscriptions = Some(iter.map(Into::into).collect());
			}
			Some(subscriptions) => {
				subscriptions.extend(iter.map(Into::into));
			}
		};

		self
	}

	/// Builds the config, consuming the builder.
	pub fn build(self, service_name: impl Into<String>, service_port: impl Into<u16>) -> Config {
		Config {
			service_name: service_name.into(),
			service_port: service_port.into(),
			timeout: self.timeout.unwrap_or(Config::DEFAULT_TIMEOUT),
			buffer: self.buffer.unwrap_or(Config::DEFAULT_BUFFER),
			throttle: self.throttle.unwrap_or(Config::DEFAULT_THROTTLE),
			heartbeat: self
				.heartbeat
				.unwrap_or(Config::DEFAULT_HEARTBEAT),
			auth: self.auth.unwrap_or_default(),
			precision_time: self
				.precision_time
				.unwrap_or(Config::DEFAULT_PRECISION),
			precision_position: self
				.precision_position
				.unwrap_or(Config::DEFAULT_PRECISION),
			precision_vector: self
				.precision_vector
				.unwrap_or(Config::DEFAULT_PRECISION),
			subscriptions: self.subscriptions.unwrap_or_else(|| {
				Subscription::UNRESTRICTED
					.iter()
					.copied()
					.collect()
			}),
		}
	}
}

/// The config file that will be writting to your CS:GO install.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "This file has been generated automatically. Do not edit.")]
pub struct ConfigFile {
	uri: String,
	timeout: f64,
	buffer: f64,
	throttle: f64,
	heartbeat: f64,
	auth: HashMap<String, String>,
	output: Precision,
	data: Data,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Precision {
	precision_time: u8,
	precision_position: u8,
	precision_vector: u8,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Data {
	map_round_wins: bool,
	map: bool,
	player_id: bool,
	player_match_stats: bool,
	player_state: bool,
	player_weapons: bool,
	provider: bool,
	round: bool,
	allgrenades: bool,
	allplayers_id: bool,
	allplayers_match_stats: bool,
	allplayers_position: bool,
	allplayers_state: bool,
	allplayers_weapons: bool,
	bomb: bool,
	phase_countdowns: bool,
	player_position: bool,
}
