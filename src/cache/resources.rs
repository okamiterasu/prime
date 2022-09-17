use std::path::Path;
use anyhow::{Result, Context};
use serde::Deserialize;
use super::load;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Export
{
    export_resources: Vec<Resource>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource
{
    pub unique_name: String,
    pub name: String,
}

pub(crate) fn load(cache: &Path, manifest: &str) -> Result<Vec<Resource>>
{
	let file = load::load(cache, manifest)
		.context("Loading manifest")?;
	serde_json::from_str(&file)
		.map(|e: Export|e.export_resources)
		.context("Parsing manifest")
}