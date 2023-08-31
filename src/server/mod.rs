//! Module containing the GSI server implementation for listening for CS:GO events.

use {
	crate::Result,
	axum::{extract::State, http::StatusCode, routing::post, Json, Router},
	std::{net::SocketAddr, sync::Arc},
	tokio::sync::{broadcast, oneshot},
	tracing::debug,
};

mod config;
pub use config::{Config, ConfigBuilder, ConfigFile, Subscription};

/// [`axum::Server`] wrapper to listen for GSI events in a background task.
///
/// You can use the [`broadcast::Receiver`] that's returned by the [`Server::start`] method or call
/// [`Server::subscribe`] to receive these events.
#[allow(missing_debug_implementations)]
pub struct Server {
	/// The config for this server.
	config: Config,

	/// [`broadcast::Sender`] for passing along incoming events.
	event_emitter: broadcast::Sender<crate::Event>,
}

impl Server {
	/// Constructs a new [`Server`] with a message buffer of `MESSAGES` and the given conifg.
	///
	/// The returned [`broadcast::Receiver`] can be used to listen to incoming events.
	#[allow(clippy::new_ret_no_self)]
	pub fn new<const MESSAGES: usize>(config: Config) -> Self {
		let (event_emitter, _) = broadcast::channel(MESSAGES);
		Self { config, event_emitter }
	}

	/// Get an additional [`broadcast::Receiver`] handle for events emitted by this server.
	///
	/// After starting the server new handles can be obtained via the
	/// [`broadcast::Receiver::resubscribe`] method.
	pub fn subscribe(&self) -> broadcast::Receiver<crate::Event> {
		self.event_emitter.subscribe()
	}

	/// Starts a background task with the server running.
	///
	/// You can use the returned [`KillSignal`] to stop the server later.
	pub fn start(self) -> Result<(broadcast::Receiver<crate::Event>, KillSignal)> {
		let event_listener = self.subscribe();
		let (kill_signal, rx) = oneshot::channel();

		let addr = SocketAddr::from(([127, 0, 0, 1], self.config.service_port));
		let router = Router::new()
			.route("/", post(Self::handler))
			.with_state(Arc::new(self));

		let server = axum::Server::bind(&addr)
			.serve(router.into_make_service())
			.with_graceful_shutdown(async {
				_ = rx.await;
			});

		tokio::task::spawn(server);

		Ok((event_listener, KillSignal(kill_signal)))
	}

	async fn handler(
		State(state): State<Arc<Self>>,
		Json(event): Json<crate::Event>,
	) -> StatusCode {
		debug!("received message");
		_ = state.event_emitter.send(event);
		StatusCode::OK
	}
}

/// Signal for shutting down the [`Server`] you got this from.
#[allow(missing_debug_implementations)]
pub struct KillSignal(oneshot::Sender<()>);

impl KillSignal {
	/// Shuts down the [`Server`] you got this signal from.
	pub fn kill(self) {
		self.0
			.send(())
			.expect("Server stopped without sending kill signal");
	}
}
