use {
	serde::{Deserialize, Serialize},
	std::{
		collections::{HashMap, HashSet},
		fmt::Debug,
		path::PathBuf,
		time::Duration,
	},
};

/// The different events one can subscribe to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Subscription {
	/// history of round wins
	MapRoundWins,
	/// mode, map, phase, team scores
	Map,
	/// steamid
	PlayerID,
	/// scoreboard info
	PlayerMatchStats,
	/// armor, flashed, equip_value, health, etc.
	PlayerState,
	/// list of player weapons and weapon state
	PlayerWeapons,
	/// info about the game providing info
	Provider,
	/// round phase and the winning team
	Round,
	/// grenade effecttime, lifetime, owner, position, type, velocity
	AllGrenades,
	/// the steam id of each player
	AllPlayersID,
	/// the scoreboard info for each player
	AllPlayersMatchStats,
	/// player_position but for each player
	AllPlayersPosition,
	/// the player_state for each player
	AllPlayersState,
	/// the player_weapons for each player
	AllPlayersWeapons,
	/// location of the bomb, who's carrying it, dropped or not
	Bomb,
	/// time remaining in tenths of a second, which phase
	PhaseCountdowns,
	/// forward direction, position for currently spectated player
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

/// Builder for a [`GSIConfig`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GSIConfigBuilder {
	service_name: String,
	timeout: Option<Duration>,
	buffer: Option<Duration>,
	throttle: Option<Duration>,
	heartbeat: Option<Duration>,
	auth: HashMap<String, String>,
	precision_time: Option<u8>,
	precision_position: Option<u8>,
	precision_vector: Option<u8>,
	subscriptions: HashSet<Subscription>,
}

impl GSIConfigBuilder {
	/// Construct a new config builder.
	pub fn new(service_name: impl Into<String>) -> Self {
		Self {
			service_name: service_name.into(),
			timeout: None,
			buffer: None,
			throttle: None,
			heartbeat: None,
			auth: HashMap::new(),
			precision_time: None,
			precision_position: None,
			precision_vector: None,
			subscriptions: HashSet::new(),
		}
	}

	/// Set the timeout for requests. Default is 1s.
	pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
		self.timeout = Some(timeout);
		self
	}

	/// Minimum wait time between sending updates. Default is 100ms.
	pub fn buffer(&mut self, buffer: Duration) -> &mut Self {
		self.buffer = Some(buffer);
		self
	}

	/// Minimum wait between responding to one update and sending the next one. Default is 1s.
	pub fn throttle(&mut self, throttle: Duration) -> &mut Self {
		self.throttle = Some(throttle);
		self
	}

	/// Minimum time between updates. Default is 30s.
	pub fn heartbeat(&mut self, heartbeat: Duration) -> &mut Self {
		self.heartbeat = Some(heartbeat);
		self
	}

	/// Authorization key/value pair.
	pub fn auth<K, V>(&mut self, key: K, value: V) -> &mut Self
	where
		K: Into<String>,
		V: Into<String>,
	{
		self.auth
			.insert(key.into(), value.into());
		self
	}

	/// Digits after the decimal point in time values. Default is 3.
	pub fn precision_time(&mut self, precision_time: u8) -> &mut Self {
		self.precision_time = Some(precision_time);
		self
	}

	/// Digits after the decimal point in position values. Default is 3.
	pub fn precision_position(&mut self, precision_position: u8) -> &mut Self {
		self.precision_position = Some(precision_position);
		self
	}

	/// Digits after the decimal point in vector values. Default is 3.
	pub fn precision_vector(&mut self, precision_vector: u8) -> &mut Self {
		self.precision_vector = Some(precision_vector);
		self
	}

	/// Subscribe to a single event.
	pub fn subscribe(&mut self, subscription: Subscription) -> &mut Self {
		self.subscriptions.insert(subscription);
		self
	}

	/// Subscribe to multiple events.
	pub fn subscribe_multiple<S, Iter>(&mut self, subscriptions: Iter) -> &mut Self
	where
		S: Into<Subscription>,
		Iter: IntoIterator<Item = S>,
	{
		self.subscriptions.extend(
			subscriptions
				.into_iter()
				.map(|sub| sub.into()),
		);
		self
	}

	/// Build the config.
	pub fn build(self) -> GSIConfig {
		self.into()
	}
}

/// Configuration for CS:GO's GSI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GSIConfig {
	service_name: String,
	timeout: Duration,
	buffer: Duration,
	throttle: Duration,
	heartbeat: Duration,
	auth: HashMap<String, String>,
	precision_time: u8,
	precision_position: u8,
	precision_vector: u8,
	subscriptions: HashSet<Subscription>,
}

impl From<GSIConfigBuilder> for GSIConfig {
	fn from(value: GSIConfigBuilder) -> Self {
		Self {
			service_name: value.service_name,
			timeout: value
				.timeout
				.unwrap_or_else(|| Duration::from_millis(1000)),
			buffer: value
				.buffer
				.unwrap_or_else(|| Duration::from_millis(100)),
			throttle: value
				.throttle
				.unwrap_or_else(|| Duration::from_millis(1000)),
			heartbeat: value
				.heartbeat
				.unwrap_or_else(|| Duration::from_secs(30)),
			auth: value.auth,
			precision_time: value.precision_time.unwrap_or(3),
			precision_position: value.precision_position.unwrap_or(3),
			precision_vector: value.precision_vector.unwrap_or(3),
			subscriptions: value.subscriptions,
		}
	}
}

impl GSIConfig {
	/// Install GSI info into CS:GO config path.
	#[tracing::instrument]
	pub fn install_into<P: Into<PathBuf> + Debug>(
		&self, cfg_folder: P, port: u16,
	) -> crate::Result<ConfigFile> {
		let mut cfg_path = cfg_folder.into();
		cfg_path.push(&format!("gamestate_integration_{}.cfg", self.service_name));

		let config = ConfigFile::new(self, port);
		let config_vdf = vdf_serde::to_string(&config)?;

		std::fs::write(cfg_path, config_vdf.as_bytes())?;

		Ok(config)
	}
}

#[allow(missing_docs)]
#[derive(Debug, Serialize)]
struct Precision {
	precision_time: u8,
	precision_position: u8,
	precision_vector: u8,
}

#[allow(missing_docs)]
#[derive(Debug, Serialize)]
struct Data {
	map_round_wins: bool,
	map: bool,
	player_id: bool,
	player_match_stats: bool,
	player_state: bool,
	player_weapons: bool,
	provider: bool,
	round: bool,

	// Below this line must be spectating or observing
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

#[allow(missing_docs)]
#[derive(Debug, Serialize)]
#[serde(rename = "Auto-generated by csgo_gsi.")]
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

impl ConfigFile {
	pub fn new(config: &GSIConfig, port: u16) -> Self {
		let GSIConfig {
			service_name: _,
			timeout,
			buffer,
			throttle,
			heartbeat,
			auth,
			precision_time,
			precision_position,
			precision_vector,
			subscriptions,
		} = config;

		Self {
			uri: format!("http://127.0.0.1:{port}"),
			timeout: timeout.as_secs_f64(),
			buffer: buffer.as_secs_f64(),
			throttle: throttle.as_secs_f64(),
			heartbeat: heartbeat.as_secs_f64(),
			auth: auth.to_owned(),
			output: Precision {
				precision_time: *precision_time,
				precision_position: *precision_position,
				precision_vector: *precision_vector,
			},
			data: Data {
				map_round_wins: subscriptions.contains(&Subscription::MapRoundWins),
				map: subscriptions.contains(&Subscription::Map),
				player_id: subscriptions.contains(&Subscription::PlayerID),
				player_match_stats: subscriptions.contains(&Subscription::PlayerMatchStats),
				player_state: subscriptions.contains(&Subscription::PlayerState),
				player_weapons: subscriptions.contains(&Subscription::PlayerWeapons),
				provider: subscriptions.contains(&Subscription::Provider),
				round: subscriptions.contains(&Subscription::Round),
				allgrenades: subscriptions.contains(&Subscription::AllGrenades),
				allplayers_id: subscriptions.contains(&Subscription::AllPlayersID),
				allplayers_match_stats: subscriptions.contains(&Subscription::AllPlayersMatchStats),
				allplayers_position: subscriptions.contains(&Subscription::AllPlayersPosition),
				allplayers_state: subscriptions.contains(&Subscription::AllPlayersState),
				allplayers_weapons: subscriptions.contains(&Subscription::AllPlayersWeapons),
				bomb: subscriptions.contains(&Subscription::Bomb),
				phase_countdowns: subscriptions.contains(&Subscription::PhaseCountdowns),
				player_position: subscriptions.contains(&Subscription::PlayerPosition),
			},
		}
	}
}
