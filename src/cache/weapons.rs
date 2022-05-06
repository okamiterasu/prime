use std::path::{Path};
use serde::Deserialize;
use super::load;

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

pub(crate) fn load(cache: &Path, manifest: &str) -> anyhow::Result<Vec<Weapon>>
{
	let file = load::load(cache, manifest)?;
	let parsed: Export = serde_json::from_str(&file)?;
	Ok(parsed.export_weapons)
}