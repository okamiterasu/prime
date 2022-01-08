use std::io;
use std::path::{PathBuf, Path};

use druid::im::Vector;
use rusqlite::{params, Connection};
use druid::{AppLauncher, WindowDesc};

mod live;
mod recipes;
mod frames;
mod weapons;
mod setup;
mod ui;
mod db;
mod relics;

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
	
	let window = WindowDesc::new(ui::builder)
		.title("Prime Tracker")
		.window_size((800.0, 600.0));
	
	let endpoints = endpoints(&cache_dir)?;
	let mut db = setup::db(&cache_dir.join("db.sqlite")).unwrap();
	setup::recipes(&cache_dir, &endpoints, &mut db).unwrap();
	setup::warframes(&cache_dir, &endpoints, &mut db).unwrap();
	setup::weapons(&cache_dir, &endpoints, &mut db).unwrap();
	setup::relics(&cache_dir, &endpoints, &mut db).unwrap();

	let mut ui_state = ui::State::default();
	ui_state.db_path = db.path().unwrap().to_owned();
	ui_state.tracked_path = cache_dir.join("tracked.json");
	ui_state.tracked_recipes = load_tracked(&ui_state.tracked_path)?;

	AppLauncher::with_window(window)
		.use_simple_logger()
		.delegate(ui::Delegate)
		.launch(ui_state).unwrap();

	Ok(())
}

fn load_tracked(path: &Path) -> io::Result<Vector<ui::Tracked>>
{
	use std::io::BufReader;
	use std::fs::File;
	if !path.exists()
	{
		return Ok(Vector::new())
	}
	let reader = BufReader::new(File::open(path)?);
	let parsed: Vec<ui::Tracked> = serde_json::from_reader(reader).unwrap_or_default();
	Ok(parsed.into())
}