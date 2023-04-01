# csgo_gsi - GSI Implementation for Rust

Rust library for [CS:GO's GSI](https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration).

I made this library because [csgo-gsi](https://crates.io/crates/csgo-gsi)'s `GSIServer` is not
thread-safe (which makes 0 sense). This is simply a port of `csgo-gsi` with some small API
changes + a `Send` + `Sync` `GSIServer` :)
