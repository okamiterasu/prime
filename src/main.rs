use std::{path::{PathBuf, Path}, collections::HashMap};

use anyhow::{Result, Error, anyhow, bail, Context};
use eframe::egui;

use structures::{Data, CommonName, UniqueName, Count};
use crate::recipe::Recipe;
use crate::requirement::Requirement;

mod live;
mod ui;
mod cache;
mod structures;
mod requirement;
mod recipe;

#[cfg(target_os = "windows")]
fn cache_dir() -> Result<PathBuf>
{
	let home = std::env::var("UserProfile")?;
	let mut path = PathBuf::from(home);
	path.push("primes/");
	Ok(path)
}

#[cfg(target_os = "linux")]
fn cache_dir() -> Result<PathBuf>
{
	let home = std::env::var("HOME")?;
	let mut path = PathBuf::from(home);
	path.push(".cache/primes/");
	Ok(path)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord,Hash)]
pub enum Rarity
{
	COMMON,
	UNCOMMON,
	RARE
}
impl Rarity
{
	pub fn as_str(&self) -> &'static str
	{
		match self
		{
			Self::COMMON=>"COMMON",
			Self::UNCOMMON=>"UNCOMMON",
			Self::RARE=>"RARE"
		}
	}
}
impl TryFrom<&str> for Rarity
{
	type Error = Error;
	fn try_from(i: &str) -> Result<Self, Self::Error>
	{
		match i
		{
			"COMMON"=>Ok(Self::COMMON),
			"UNCOMMON"=>Ok(Self::UNCOMMON),
			"RARE"=>Ok(Self::RARE),
			_=>Err(anyhow!("Unknown rarity: {}", i))
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relic
{
	name: String,
	rarity: Rarity
}

impl Relic
{
	fn new(name: &str, rarity: &str) -> Result<Self>
	{
		let rarity = Rarity::try_from(rarity)?;
		let x = Self
		{
			name: name.to_string(),
			rarity
		};
		Ok(x)
	}
}

#[derive(Debug)]
pub(crate) struct Tracked
{
	pub(crate) common_name: CommonName,
	pub(crate) unique_name: UniqueName,
	pub(crate) recipes: Vec<(Recipe, Vec<(Requirement, Count)>)>
}

impl Tracked
{
	fn new(db: &Data, unique_name: impl Into<UniqueName>) -> Result<Self>
	{
		let unique_name = unique_name.into();
		let common_name = db.resource_common_name(unique_name.clone())
			.context("searching for resource common name")?;

		let recipe_unique_names = db.recipes(unique_name.clone())
			.context("Searching for resource's recipe")?;
		if recipe_unique_names.is_empty() {bail!("Recipe not found for {unique_name}")}

		let mut recipes = Vec::with_capacity(recipe_unique_names.len());
		for recipe_unique_name in recipe_unique_names
		{
			let recipe = Recipe::new(db, recipe_unique_name.clone())?;
			let mut components = vec![];
			for (unique_name, count) in db.requirements(recipe_unique_name.clone())
				.context("Looking for recipe's requirements")?
			{
				let requirement = Requirement::new(unique_name.clone(), db)
					.with_context(||format!("Generating component data for {:?}", unique_name))?;
				components.push((requirement, count));
			}
			recipes.push((recipe, components));
		}
		Ok(Self{common_name, unique_name, recipes})
	}
}

fn main() -> Result<()>
{
	let cache_dir = cache_dir()?;
	if !cache_dir.exists() {std::fs::create_dir_all(&cache_dir)?;}

	if let Some((index, index_raw)) = check_for_manifest_updates(&cache_dir)
		.context("Checking for manifest updates")?
	{
		std::fs::write(cache_dir.join("index_en.txt"), index_raw)?;
		update_manifests(&cache_dir, &index)
			.context("Updating manifests")?;
		remove_old_manifests(&cache_dir, &index)
			.context("Removing old manifests")?;
		}
	// Worldstate has probably changed too, so update that as well.
	let dt = live::droptable()
		.context("Downloading scrape droptable")?;
	std::fs::write(cache_dir.join("droptable.html"), dt)?;

	let ws = live::worldstate()
		.context("Downloading world state")?;
	std::fs::write(cache_dir.join("worldstate.json"), ws)?;

	let mut data = Data::from_cache(&cache_dir)?;

	let (tracked, owned) = cache::load_state(&cache_dir.join("tracked.json"), &mut data)
		.context("Loading tracked file")?;

	let opts = eframe::NativeOptions
	{
		initial_window_size: Some(egui::Vec2::new(1024.0, 768.0)),
		..Default::default()
	};
	eframe::run_native(
		"Recipe Tracker",
		opts,
		Box::new(|_cc| Box::new(ui::App::with_state(data, tracked, owned, cache_dir))));
	Ok(())
}

// Downloads the index from live and compares it to the local version.
// If they are not the same, returns both raw and parsed live version.
fn check_for_manifest_updates(dir: &Path) -> Result<Option<(HashMap<String, String>, String)>>
{
	fn parse_live_index(index: &str) -> HashMap<String, String>
	{
		index.lines()
			.map(|l|(&l[0..l.len()-26], l))
			.map(|(k, v)|(k.to_owned(), v.to_owned()))
			.collect()
	}

	let live_index_raw = live::index()
		.context("loading live index")?;
	let live_index = parse_live_index(&live_index_raw);
	let local_index = cache::load_index(&dir.join("index_en.txt"))
		.unwrap_or_default();
	let is_different = live_index != local_index;
	Ok(is_different.then_some((live_index, live_index_raw)))
}



fn update_manifests(dir: &Path, index: &HashMap<String, String>) -> Result<()>
{
	for manifest in index.values()
	{
		let path = dir.join(manifest);
		if !path.exists()
		{
			let m = live::manifest(manifest)
				.with_context(||format!("Downloading manifest: {manifest}"))?;
			std::fs::write(&path, m)?;
		}
	}
	Ok(())
}

fn remove_old_manifests(dir: &Path, index: &HashMap<String, String>) -> Result<()>
{
	for file in std::fs::read_dir(dir)?.flatten()
	{
		let file_name = file.file_name();
		let file_name = file_name.to_str()
			.ok_or_else(||anyhow!("Non-utf8 string"))?;
		
		if file_name.starts_with("Export") 
		&& file_name != index[&file_name[0..file_name.len()-26]]
		{
			std::fs::remove_file(file.path())
				.with_context(||format!("Deleting file: {:?}", file.file_name()))?;
		}
	}
	Ok(())
}