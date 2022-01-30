use std::path::PathBuf;

use druid::{AppLauncher, WindowDesc};

mod live;
mod recipes;
mod warframes;
mod weapons;
mod setup;
mod ui;
mod db;
mod relics;
mod resources;
mod persistance;
mod droptable;
mod worldstate;

#[cfg(target_os = "windows")]
fn cache_dir() -> PathBuf
{
	let home = std::env::var("UserProfile").expect("HOME env not set");
	let mut path = PathBuf::from(home);
	path.push("primes/");
	path
}

#[cfg(target_os = "linux")]
fn cache_dir() -> PathBuf
{
	let home = std::env::var("HOME").expect("HOME env not set");
	let mut path = PathBuf::from(home);
	path.push(".cache/primes/");
	path
}

fn main() -> anyhow::Result<()>
{
	let cache_dir = cache_dir();
	if !cache_dir.exists()
	{
		std::fs::create_dir(&cache_dir)?;
	}
	let  db = db::open(&cache_dir, "db.sqlite", false)?;

	let ui_state = persistance::load(&cache_dir.join("tracked.json"), db)?;

	let window = WindowDesc::new(ui::builder)
		.title("Prime Tracker")
		.window_size((800.0, 800.0));

	AppLauncher::with_window(window)
		.use_simple_logger()
		.delegate(ui::Delegate)
		.launch(ui_state)?;

	Ok(())
}

