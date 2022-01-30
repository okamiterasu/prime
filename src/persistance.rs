use std::io;
use std::path::Path;
use druid::im::Vector;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use super::ui;
use super::db;

#[derive(Eq, PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
struct Tracked
{
	unique_name: String,
	recipe: Component,
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
			recipe: i.recipe.into(),
			components
		}
	}
}

impl Tracked
{
	pub fn into_ui(mut self, db: &Connection) -> rusqlite::Result<ui::Tracked>
	{
		let common_name = db::common_name(db, &self.unique_name)?;
		let requires = std::mem::take(&mut self.components)
			.into_iter()
			.map(|c|{
				c.into_ui(
					common_name.as_ref(),
					&self.recipe.unique_name,
					db,
					false)}
			).flatten()
			.collect();
		let recipe_unique_name = &self.recipe.unique_name.to_owned();
		let tracked = ui::Tracked
		{
			unique_name: self.unique_name,
			recipe: self.recipe.into_ui(
				common_name.as_ref(),
				recipe_unique_name,
				db,
				true)?,
			common_name,
			requires
		};
		Ok(tracked)
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
	pub fn into_ui(self, parent_common_name: Option<&String>, parent_recipe_unique_name: &str, db: &Connection, main_bp: bool) -> rusqlite::Result<ui::Component>
	{
		let mut com = ui::Component::new(
			db,
			parent_recipe_unique_name,
			self.unique_name,
			parent_common_name.unwrap(),
			main_bp)?;
		com.owned = self.owned;
		Ok(com)
	}
}

pub fn load(tracked_path: &Path, db: Connection) -> io::Result<ui::State>
{
	let contents = std::fs::read_to_string(tracked_path)?;
	let parsed: Vec<Tracked> = serde_json::from_str(&contents)?;
	let tracked_recipes: Vector<_> = parsed.into_iter()
		.map(|t|t.into_ui(&db))
		.flatten()
		.collect();
	let ui_state = ui::State::new(db, tracked_recipes, tracked_path);
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