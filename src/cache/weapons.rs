use std::path::Path;
use anyhow::{Result, Context};
use serde::Deserialize;
use super::manifest;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Export
{
	export_weapons: Vec<Weapon>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Weapon
{
	pub unique_name: String,
	pub name: String,
}

pub fn load(cache: &Path, manifest: &str) -> Result<Vec<Weapon>>
{
	let file = manifest::load(cache, manifest)
		.context("Loading manifest")?;
	serde_json::from_str(&file)
		.map(|e: Export|e.export_weapons)
		.context("Parsing manifest")
}