use std::io;
use std::path::{PathBuf, Path};

use druid::im::Vector;
use rusqlite::{params, Connection};
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

fn endpoints(path: &Path) -> io::Result<Vec<String>>
{
	let index_path = path.join("index_en.txt");
	if !index_path.exists()
	{
		let index = live::index()?;
		std::fs::write(&index_path, index)?;
	}
	let contents = std::fs::read_to_string(index_path)?;
	let endpoints = contents
		.lines()
		.map(|e|e.to_owned())
		.collect();
	Ok(endpoints)
}

fn main() -> io::Result<()>
{
	let cache_dir = cache_dir();
	if !cache_dir.exists()
	{
		std::fs::create_dir(&cache_dir)?;
	}
	
	
	
	let endpoints = endpoints(&cache_dir)?;
	let mut db = setup::db(&cache_dir.join("db.sqlite")).unwrap();
	setup::resources(&cache_dir, &endpoints, &mut db).unwrap();
	setup::warframes(&cache_dir, &endpoints, &mut db).unwrap();
	setup::weapons(&cache_dir, &endpoints, &mut db).unwrap();
	setup::relics(&cache_dir, &endpoints, &mut db).unwrap();
	setup::recipes(&cache_dir, &endpoints, &mut db).unwrap();

	let ui_state = persistance::load(&cache_dir.join("tracked.json"), &mut db)?;

	let window = WindowDesc::new(ui::builder)
		.title("Prime Tracker")
		.window_size((800.0, 600.0));

	AppLauncher::with_window(window)
		.use_simple_logger()
		.delegate(ui::Delegate)
		.launch(ui_state).unwrap();

	Ok(())
}

