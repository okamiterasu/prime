use std::path::{Path};

use anyhow::{Result, Context, anyhow};
use serde::{Deserialize};

use super::manifest;
use crate::relic::Rarity;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
struct Export
{
	export_relic_arcane: Vec<RelicArcane>
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct RelicArcane
{
	pub unique_name: String,
	pub name: String,
	pub relic_rewards: Option<Vec<Reward>>
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Relic
{
	pub unique_name: String,
	pub name: String,
	pub relic_rewards: Vec<Reward>
}

impl TryFrom<RelicArcane> for Relic
{
	type Error = anyhow::Error;

	fn try_from(value: RelicArcane) -> Result<Self, Self::Error>
	{
		if let Some(relic_rewards) = value.relic_rewards
		{
			Ok(Self
			{
				unique_name: value.unique_name,
				name: value.name,
				relic_rewards
			})
		}
		else
		{
			Err(anyhow!("RelicArcane {} was not a relic", value.name))
		}
	}
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reward
{
	pub reward_name: String,
	pub rarity: Rarity,
}

pub(crate) fn load(cache: &Path, manifest: &str) -> Result<Vec<Relic>>
{
	let file = manifest::load(cache, manifest)
		.context("Loading manifest")?;
	let parsed: Export = serde_json::from_str(&file)
		.context("Parsing manifest")?;
	let relics = parsed.export_relic_arcane.into_iter()
		.flat_map(|r|r.try_into())
		.collect();
	Ok(relics)
}