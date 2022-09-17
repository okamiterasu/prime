use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;
use super::load;

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

pub(crate) fn load(cache: &Path, manifest: &str) -> Result<Vec<Recipe>>
{
	let file = load::load(cache, manifest)
		.context("Loading manifest")?;
	serde_json::from_str(&file)
		.map(|e: Export|e.export_recipes)
		.context("Parsing manifest")
}