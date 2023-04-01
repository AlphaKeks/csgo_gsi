//! Rust library for [CS:GO's GSI](https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration).
//!
//! I made this library because [csgo-gsi](https://crates.io/crates/csgo-gsi)'s `GSIServer` is not
//! thread-safe (which makes 0 sense). This is simply a port of `csgo-gsi` with some small API
//! changes + a [`Send`] + [`Sync`] [`GSIServer`] :)
#![warn(rust_2018_idioms, missing_docs, missing_debug_implementations)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![deny(clippy::correctness, clippy::perf)]

/// Crate-wide error type.
mod error;
pub use error::{Error, Result};

/// Module containing all the datastructures for events received from CS:GO.
pub mod event;
pub use event::Event;

/// Module for the GSI Configurations.
mod config;
pub use config::{GSIConfig, GSIConfigBuilder, Subscription};

/// Module for searching the file system for a CS:GO installation folder.
pub(crate) mod install_dir;

/// Module for the server that is listening to CS:GO game updates.
mod server;
pub use server::GSIServer;
