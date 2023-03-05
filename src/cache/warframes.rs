use std::path::Path;
use anyhow::{Result, Context};
use serde::Deserialize;
use super::manifest;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Export
{
	export_warframes: Vec<Warframe>,
	// export_abilities: Value
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Warframe
{
	pub unique_name: String,
	pub name: String,
	pub product_category: String
}

pub(crate) fn load(cache: &Path, manifest: &str) -> Result<Vec<Warframe>>
{
	let file = manifest::load(cache, manifest)
		.context("Loading manifest")?;
	serde_json::from_str(&file)
		.map(|e: Export|e.export_warframes)
		.context("Parsing manifest")
}