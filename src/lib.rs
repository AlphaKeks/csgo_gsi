//! Rust library for [CS:GO's GSI](https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration).
#![warn(rust_2018_idioms, missing_docs, missing_debug_implementations)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![deny(clippy::correctness, clippy::perf)]

/// Crate-wide error type.
mod error;
pub use error::{Error, Result};

/// Module containing all the datastructures for events received from CS:GO.
pub mod event;

/// Module for the GSI Configurations.
mod config;
pub use config::{GSIConfig, GSIConfigBuilder, Subscription};

/// Module for searching the file system for a CS:GO installation folder.
pub(crate) mod install_dir;

/// Module for the server that is listening to CS:GO game updates.
mod server;
pub use server::GSIServer;
