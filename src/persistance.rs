use std::io;
use std::path::Path;
use druid::im::Vector;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use super::ui;
use super::db;

#[derive(Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
struct Tracked
{
	unique_name: String,
	recipe: (String, u32),
	components: Vec<Component>
}

impl From<ui::Tracked> for Tracked
{
	fn from(i: ui::Tracked) -> Self
	{
		let components = i.requires.into_iter()
			.map(|c|c.into())
			.collect();
		Self
		{
			unique_name: i.unique_name,
			recipe: i.recipe,
			components
		}
	}
}

impl Tracked
{
	pub fn into_ui(self, db: &mut Connection) -> ui::Tracked
	{
		let common_name = db::common_name(db, &self.unique_name).unwrap();
		let requires = self.components.into_iter()
			.map(|c|c.into_ui(common_name.as_ref(), &self.recipe.0, db))
			.collect();
		ui::Tracked
		{
			unique_name: self.unique_name,
			recipe: self.recipe,
			common_name,
			requires
		}
	}
}

#[derive(Eq, PartialEq,Clone, Default, Deserialize, Serialize, Debug)]
struct Component
{
	unique_name: String,
	owned: u32,
}

impl From<ui::Component> for Component
{
	fn from(i: ui::Component) -> Self
	{
		Self
		{
			unique_name: i.unique_name,
			owned: i.owned
		}
	}
}

impl Component
{
	pub fn into_ui(self, parent_common_name: Option<&String>, parent_recipe_unique_name: &str, db: &mut Connection) -> ui::Component
	{
		dbg!(&self);
		let common_name = db::common_name(db, &self.unique_name)
			.unwrap()
			.as_ref()
			.map(|n|n.trim_start_matches(parent_common_name.unwrap()))
			.map(|n|n.trim_start())
			.map(|n|n.to_owned());
		let count = db::how_many_needed(db, parent_recipe_unique_name, &self.unique_name).unwrap();
		let relics = db::relics(db, &self.unique_name).unwrap().into();
		ui::Component
		{
			common_name,
			unique_name: self.unique_name,
			owned: self.owned,
			count,
			relics
		}
	}
}

pub fn load(tracked_path: &Path, db: &mut Connection) -> io::Result<ui::State>
{
	use std::io::BufReader;
	use std::fs::File;
	let mut ui_state = ui::State::default();
	ui_state.db_path = db.path().expect("Empty DB Path").to_owned();
	ui_state.tracked_path = tracked_path.to_owned();
	if !tracked_path.exists()
	{
		return Ok(ui_state)
	}

	let reader = BufReader::new(File::open(tracked_path)?);
	let parsed: Vec<Tracked> = serde_json::from_reader(reader)?;
	ui_state.tracked_recipes = parsed.into_iter().map(|t|t.into_ui(db)).collect();
	Ok(ui_state)
}

pub fn save(tracked_path: &Path, data: &Vector<ui::Tracked>) -> io::Result<()>
{
	let tracked: Vec<Tracked> = data.iter()
		.cloned()
		.map(|t|t.into())
		.collect();
	let file = std::fs::File::create(tracked_path)?;
	let mut buf = io::BufWriter::new(file);
	serde_json::to_writer(&mut buf, &tracked)?;
	Ok(())
}