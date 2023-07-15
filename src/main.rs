use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use eframe::egui;

use crate::recipe::Recipe;
use crate::requirement::Requirement;
use crate::structures::{Data, CommonName, UniqueName, Count};

mod cache;
mod item_view;
mod live;
mod recipe;
mod relic;
mod requirement;
mod structures;
mod ui;

const ICON_DATA: &[u8] = include_bytes!("../icon.png");

#[cfg(target_os = "windows")]
fn cache_dir() -> Result<PathBuf>
{
	#[cfg(target_os = "windows")]
	const CACHE_DIR: &str = "primes/";
	#[cfg(target_os = "linux")]
	const CACHE_DIR: &str = ".cache/primes/";

	dirs::home_dir()
		.context("Could not find Home dir")
		.map(|h|h.join(CACHE_DIR))
}

#[derive(Debug)]
pub struct Tracked
{
	common_name: CommonName,
	unique_name: UniqueName,
	recipes: Vec<(Recipe, Vec<(Requirement, Count)>)>
}

impl Tracked
{
	fn new(db: &Data, unique_name: impl Into<UniqueName>) -> Result<Self>
	{
		let unique_name = unique_name.into();
		let common_name = db.resource_common_name(unique_name.clone())
			.context("searching for resource common name")?;

		let mut recipes = vec![];
		for recipe_unique_name in db.recipes(unique_name.clone())
		{
			let recipe = Recipe::new(db, recipe_unique_name.clone())?;
			let mut components = vec![];
			for (unique_name, count) in db.requirements(recipe_unique_name.clone())
			{
				let requirement = Requirement::new(unique_name.clone(), db)
					.with_context(||format!("Generating component data for {unique_name:?}"))?;
				components.push((requirement, count));
			}
			recipes.push((recipe, components));
		}

		if recipes.is_empty() {bail!("Recipe not found for {unique_name}")}
		Ok(Self{common_name, unique_name, recipes})
	}
}

fn main() -> Result<()>
{
	let cache_dir = cache_dir()?;
	if !cache_dir.exists() {std::fs::create_dir_all(&cache_dir)?;}

	update_index(&cache_dir)
		.context("Checking for manifest updates")?;

	let droptable_path = cache_dir.join("droptable.html");
	let droptable = live::droptable()
		.context("Downloading scrape droptable")?;
	std::fs::write(droptable_path, droptable)?;

	let worldstate_path = cache_dir.join("worldstate.json");
	let worldstate = live::worldstate()
		.context("Downloading world state")?;
	std::fs::write(worldstate_path, worldstate)?;

	let mut data = Data::from_cache(&cache_dir)?;

	let tracked_path = cache_dir.join("tracked.json");
	let (tracked, owned) = cache::load_state(&tracked_path, &mut data)
		.context("Loading tracked file")?;

	let opts = eframe::NativeOptions
	{
		initial_window_size: Some(egui::Vec2::new(1024.0, 768.0)),
		icon_data: eframe::IconData::try_from_png_bytes(ICON_DATA).ok(),
		..Default::default()
	};
	eframe::run_native(
		"Recipe Tracker",
		opts,
		Box::new(|_cc| Box::new(ui::App::with_state(data, tracked, owned, cache_dir)))).unwrap();
	Ok(())
}

fn update_index(dir: &Path) -> Result<()>
{
	let index_path = dir.join("index_en.txt.lzma");
	let index = live::index()
		.context("Downloading new index")?;

	let parsed_index = cache::parse_index(&mut std::io::BufReader::new(index.as_slice()))?;
	remove_old_manifests(dir, &parsed_index)?;

	std::fs::write(index_path, index)
		.context("Writing new index to disk")?;
	Ok(())
}

fn remove_old_manifests(dir: &Path, index: &HashMap<String, String>) -> Result<()>
{
	for file in std::fs::read_dir(dir)?
	{
		let file = file?;
		let file_name = file.file_name();
		let file_name = file_name.to_str()
			.ok_or_else(||anyhow!("Non-utf8 string"))?;

		if file_name.starts_with("Export")
			&& file_name != index[&file_name[0..file_name.len()-26]]
		{
			println!("Deleting stale manifest: {file_name}");
			std::fs::remove_file(file.path())
				.with_context(||format!("Deleting file: {:?}", file.file_name()))?;
		}
	}
	Ok(())
}