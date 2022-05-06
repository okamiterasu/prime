use std::path::PathBuf;
use anyhow::{Result, Error, anyhow};
use db::Database;
use eframe::egui;

mod live;
mod ui;
mod db;
mod cache;

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
		let resurgence_relics = Default::default();
		Ok(Self{common_name, unique_name: unique_name, active_relics, resurgence_relics})
	}

	fn with_common_name(db: &mut Database, unique_name: String, common_name: Option<String>) -> Result<Self>
	{
		let active_relics = db.active_recipe_relics(&unique_name)?;
		let resurgence_relics = Default::default();
		Ok(Self{common_name, unique_name: unique_name, active_relics, resurgence_relics})
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
		let resurgence_relics = Default::default();
		let recipe = db
			.recipe(&unique_name)
			.map(|r|Recipe::new(db, r).unwrap())
			.ok();
		Ok(Self{unique_name, common_name, count, active_relics, resurgence_relics, recipe})
	}
}

#[derive(Debug)]
pub(crate) struct Tracked
{
	pub(crate) common_name: Option<String>,
	pub(crate) unique_name: String,
	pub(crate) recipe: Recipe,
	pub(crate) components: Vec<Component>
}

impl Tracked
{
	fn new(db: &mut Database, unique_name: String) -> Result<Self>
	{
		let common_name = db.item_common_name(&unique_name)?;
		let recipe_unique_name = db.recipe(&unique_name)?;
		let recipe = Recipe::with_common_name(
			db,
			recipe_unique_name.clone(),
			common_name.as_ref().map(|c|format!("{} Blueprint", c)))?;
		let components = db.requirements(&recipe_unique_name)?
			.into_iter()
			.map(|c|Component::new(db, c.0, &recipe.unique_name).unwrap())
			.collect();
		Ok(Self{common_name, unique_name, recipe, components})
	}
}

fn main() -> Result<()>
{
	let cache_dir = cache_dir();
	if !cache_dir.exists()
	{
		std::fs::create_dir(&cache_dir)?;
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

