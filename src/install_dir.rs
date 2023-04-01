use {
	crate::{Error, Result},
	std::path::PathBuf,
};

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("Unsupported operating system. This crate only works on Linux, MacOS and Windows.");

#[cfg(unix)]
pub fn find_steam_library() -> Result<Option<PathBuf>> {
	let mut steam_path = PathBuf::from(std::env::var("HOME")?);
	#[cfg(target_os = "linux")]
	steam_path.extend(&[".local", "share", "Steam"]);
	#[cfg(target_os = "macos")]
	steam_path.extend(&[
		"Library", "Application Support", "Steam",
	]);

	if !steam_path.exists() {
		return Ok(None);
	}

	Ok(Some(steam_path))
}

#[cfg(windows)]
pub fn find_steam_library() -> Result<Option<PathBuf>> {
	const DEFAULT_PATH: &str = r#"C:\Program Files (x86)\Steam\steamapps"#;
	let steam_path = PathBuf::from(DEFAULT_PATH);

	if !steam_path.exists() {
		return Ok(None);
	}

	Ok(Some(steam_path))
}

#[tracing::instrument]
pub fn find_cfg_folder() -> Result<PathBuf> {
	let mut csgo_folder = find_steam_library()?.ok_or(Error::NoCfgDir)?;
	csgo_folder.push("common");
	csgo_folder.push("Counter-Strike Global Offsenive");
	csgo_folder.push("csgo");
	csgo_folder.push("cfg");

	if !csgo_folder.exists() {
		return Err(Error::NoCfgDir);
	}

	Ok(csgo_folder)
}
