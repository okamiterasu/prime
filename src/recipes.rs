use std::path::{Path};

use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
struct Export
{
	export_recipes: Vec<Recipe>
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Recipe
{
	pub unique_name: String,
	pub result_type: String,
	pub ingredients: Vec<Ingredient>
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Ingredient
{
	pub item_type: String,
	pub item_count: u32,
}

pub fn parse_from_file(path: &Path) -> std::io::Result<Vec<Recipe>>
{
	let file_contents = std::fs::read_to_string(path)?;
	let escaped = file_contents
		.replace(r"\r", "")
		.replace(&['\r', '\n'][..], "");
	let parsed: Export = serde_json::from_str(&escaped)?;
	let recipes = parsed.export_recipes;
	Ok(recipes)
}