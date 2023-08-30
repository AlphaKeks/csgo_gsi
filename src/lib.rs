//! Types for [Valve's CS:GO GSI](<https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration>)
//!
//! Also comes with a `server` feature that runs [`axum`](<https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration>)
//! in the background listening for requests from CS:GO, sending them through a channel you can
//! listen on.

#![deny(clippy::correctness, clippy::perf)]
#![warn(clippy::complexity, clippy::cognitive_complexity, clippy::style)]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

pub mod csgo;
pub use csgo::Event;

#[cfg(feature = "server")]
mod error;

#[cfg(feature = "server")]
pub use error::{Error, Result};

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub use server::{Config, Server};
