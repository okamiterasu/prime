use std::{path::{PathBuf, Path}, collections::HashMap};
use anyhow::{Result, Error, anyhow, bail, Context};
use db::Database;
use eframe::egui;

mod live;
mod ui;
mod db;
mod cache;

#[cfg(target_os = "windows")]
fn cache_dir() -> Result<PathBuf>
{
	let home = std::env::var("UserProfile")?;
	let mut path = PathBuf::from(home);
	path.push("primes/");
	Ok(path)
}

#[cfg(target_os = "linux")]
fn cache_dir() -> PathBuf
{
	let home = std::env::var("HOME")?;
	let mut path = PathBuf::from(home);
	path.push(".cache/primes/");
	Ok(path)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
struct Relic
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
struct Recipe
{
	common_name: Option<String>,
	unique_name: String,
	active_relics: Vec<Relic>,
	resurgence_relics: Vec<Relic>
}

impl Recipe
{
	fn new(db: &mut Database, unique_name: String) -> Result<Self>
	{
		let common_name = db.resource_common_name(&unique_name)?;
		let active_relics = db.active_recipe_relics(&unique_name)?;
		let resurgence_relics = db.resurgence_recipe_relics(&unique_name)?;
		Ok(Self{common_name, unique_name, active_relics, resurgence_relics})
	}

	fn with_common_name(db: &mut Database, unique_name: String, common_name: Option<String>) -> Result<Self>
	{
		let active_relics = db.active_recipe_relics(&unique_name)?;
		let resurgence_relics = db.resurgence_recipe_relics(&unique_name)?;
		Ok(Self{common_name, unique_name, active_relics, resurgence_relics})
	}
}

#[derive(Debug)]
struct Component
{
	unique_name: String,
	common_name: Option<String>,
	count: u32,
	active_relics: Vec<Relic>,
	resurgence_relics: Vec<Relic>,
	recipe: Option<Recipe>
}

impl Component
{
	pub(crate) fn new(db: &mut Database, unique_name: String, recipe_unique_name: &str) -> Result<Self>
	{
		let common_name = db.resource_common_name(&unique_name)?;
		let count = db.how_many_needed(recipe_unique_name, &unique_name)?;
		let active_relics = db.active_component_relics(&unique_name)?;
		let resurgence_relics = db.resurgence_component_relics(&unique_name)?;
		let recipe = db.recipe(&unique_name)?
			.map(|r|Recipe::new(db, r))
			.transpose()?;
		Ok(Self{unique_name, common_name, count, active_relics, resurgence_relics, recipe})
	}
}

#[derive(Debug)]
pub(crate) struct Tracked
{
	pub(crate) common_name: Option<String>,
	pub(crate) unique_name: String,
	pub(crate) recipes: Vec<(Recipe, Vec<Component>)>,
	// pub(crate) recipe: Recipe,
	// pub(crate) components: Vec<Component>
}

impl Tracked
{
	fn new(db: &mut Database, unique_name: String) -> Result<Self>
	{
		let common_name = db.item_common_name(&unique_name)?;
		let recipe_unique_names = db.recipes(&unique_name)?;
		if recipe_unique_names.len() == 0
		{
			bail!("Recipe not found for {}", unique_name)
		}
		let mut recipes = vec![];
		for r in recipe_unique_names
		{
			let cn = common_name.as_ref().map(|c|format!("{} Blueprint", c));
			let recipe = Recipe::with_common_name(db, r.clone(), cn)?;
			let components = db.requirements(&r)?
				.into_iter()
				.flat_map(|c|Component::new(db, c.0, &recipe.unique_name))
				.collect();
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
		.context("Could not check for manifest updates")?
	{
		std::fs::write(cache_dir.join("index_en.txt"), &index_raw)?;
		update_manifests(&cache_dir, &index)
			.context("Could not update manifests")?;
		remove_old_manifests(&cache_dir, &index)
			.context("Could not remove old manifests")?;
		// Worldstate has probably changed too, so update that as well.
		let ws = live::droptable()
			.context("Failed to scrape droptable")?;
		std::fs::write(cache_dir.join("droptable.html"), &ws)?;
		// database is old, so delete and rebuild later.
		// TODO: clear instead of full delete. Probably faster.
		std::fs::remove_file(&cache_dir.join("db.sqlite"))?;
	}

	let mut db = match db::Database::open(&cache_dir, "db.sqlite")
	{
		Ok(db)=>db,
		Err(_)=>db::Database::create(&cache_dir, "db.sqlite")?
	};

	let (tracked, owned) = cache::load_state(&cache_dir.join("tracked.json"), &mut db)?;

	let mut opts = eframe::NativeOptions::default();
	opts.initial_window_size = Some(egui::Vec2::new(1024.0, 768.0));
	eframe::run_native(
		"Recipe Tracker",
		opts,
		Box::new(|_cc| Box::new(ui::App::with_state(db, tracked, owned, cache_dir))));
}

// Downloads the index from live and compares it to the local version.
// If they are not the same, returns both raw and parsed live version.
fn check_for_manifest_updates(dir: &Path) -> Result<Option<(HashMap<String, String>, String)>>
{
	let (live_index, live_index_raw) = live::index()
		.context("Could not load live index")?;
	let local_index_path = dir.join("index_en.txt");
	let local_index = cache::load_index(&local_index_path)
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
			let m = live::load_manifest(manifest)
				.with_context(||format!("Could not load manifest: {manifest}"))?;
			std::fs::write(&path, m)?;
		}
	}
	Ok(())
}

fn remove_old_manifests(dir: &Path, index: &HashMap<String, String>) -> Result<()>
{
	for file in std::fs::read_dir(&dir)?
	{
		let file = file?;
		let file_name = file.file_name();
		let file_name = file_name.to_str()
			.ok_or(anyhow!("Non-utf8 string"))?;
		if !file_name.starts_with("Export") {continue}
		if file_name != index[&file_name[0..file_name.len()-26]]
		{
			dbg!(file_name);
		}
	}
	Ok(())
}