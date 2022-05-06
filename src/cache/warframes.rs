use std::path::Path;
use serde::Deserialize;
use super::load;

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

pub(crate) fn load(cache: &Path, manifest: &str) -> anyhow::Result<Vec<Warframe>>
{
	let file = load::load(cache, manifest)?;
	let parsed: Export = serde_json::from_str(&file)?;
	Ok(parsed.export_warframes)
}