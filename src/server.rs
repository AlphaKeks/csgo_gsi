use {
	crate::{event::Event, install_dir, Error, GSIConfig, Result},
	axum::{
		extract::{Json, State},
		http::StatusCode,
		response::IntoResponse,
		routing::post,
		Router, Server,
	},
	std::{fmt::Debug, future::Future, net::SocketAddr, path::PathBuf, pin::Pin},
	tokio::sync::mpsc::{self, UnboundedSender},
	tracing::{error, info},
};

/// The server listening for CS:GO game updates.
#[allow(missing_debug_implementations)]
pub struct GSIServer {
	/// The port to listen on.
	port: u16,
	/// The config to use.
	config: GSIConfig,
	/// Whether the server relevant files are already in place.
	installed: bool,
	/// The registered callback funtions to execute when an event fires.
	listeners: Vec<Box<dyn FnMut(Event) + Send + Sync>>,
	/// The registered async callback funtions to execute when an event fires.
	async_listeners:
		Vec<Box<dyn FnMut(Event) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>>,
}

#[allow(unused)]
#[cfg(test)]
mod thread_safety {
	fn is_thread_safe<T: Send + Sync>() {}

	fn test_server() {
		is_thread_safe::<super::GSIServer>()
	}
}

impl GSIServer {
	/// Create a new server.
	pub fn new(config: GSIConfig, port: u16) -> Self {
		Self {
			port,
			config,
			installed: false,
			listeners: Vec::new(),
			async_listeners: Vec::new(),
		}
	}

	/// Install the server's configuration into CS:GO's cfg folder.
	pub fn install(&mut self) -> Result<&mut Self> {
		if !self.installed {
			self.install_into(install_dir::find_cfg_folder()?)?;
			self.installed = true;
			return Ok(self);
		}

		Ok(self)
	}

	/// Set the installation directory for the server.
	pub fn install_into<P: Into<PathBuf> + Debug>(&mut self, cfg_folder: P) -> Result<&mut Self> {
		if !self.installed {
			self.config
				.install_into(cfg_folder, self.port)?;
			self.installed = true;
			return Ok(self);
		}

		Ok(self)
	}

	/// Add an event listener to this server. The `cb` callback will be executed whenever an event
	/// fires.
	pub fn add_event_listener<CB>(&mut self, cb: CB)
	where
		CB: FnMut(Event) + Send + Sync + 'static,
	{
		self.listeners.push(Box::new(cb));
	}

	/// Add an async event listener to this server. The `cb` callback will be executed whenever an
	/// event fires.
	pub fn add_async_event_listener<CB>(&mut self, cb: CB)
	where
		CB: FnMut(Event) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync + 'static,
	{
		self.async_listeners.push(Box::new(cb));
	}

	/// Start the server. This will block indefinitely, so you probably want to spawn a separate
	/// thread for this.
	#[tracing::instrument(skip(self))]
	pub async fn run(mut self) -> Result<()> {
		if !self.installed {
			self.install()?;
		}

		let (sender, mut receiver) = mpsc::unbounded_channel::<Event>();

		let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

		info!("Starting server on {addr}.");
		tokio::spawn(run_server(addr, sender));

		info!("Listening for events...");
		while let Some(event) = receiver.recv().await {
			for cb in &mut self.listeners {
				cb(event.clone());
			}

			for async_cb in &mut self.async_listeners {
				async_cb(event.clone()).await;
			}
		}

		Ok(())
	}
}

/// Launches the Axum server for listening to CS:GO updates.
#[tracing::instrument]
async fn run_server(addr: SocketAddr, sender: UnboundedSender<Event>) -> Result<()> {
	let router = Router::new()
		.route("/", post(handle_update))
		.with_state(sender);

	Server::bind(&addr)
		.serve(router.into_make_service())
		.await
		.map_err(|_| Error::Axum)?;

	Ok(())
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn handle_update(
	State(sender): State<UnboundedSender<Event>>,
	Json(body): Json<Event>,
) -> impl IntoResponse {
	match sender.send(body.clone()) {
		Ok(()) => (StatusCode::OK, Json(body)),
		Err(why) => {
			error!("Failed to send event to main thread: {why:?}");
			(StatusCode::INTERNAL_SERVER_ERROR, Json(body))
		}
	}
}
