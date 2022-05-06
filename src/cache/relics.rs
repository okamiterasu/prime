use std::path::{Path};

use serde::{Deserialize};

use super::load;

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rarity
{
	COMMON,
	UNCOMMON,
	RARE
}
impl Rarity
{
	pub fn as_str(&self) -> &'static str
	{
		match self
		{
			Self::COMMON=>"COMMON",
			Self::UNCOMMON=>"UNCOMMON",
			Self::RARE=>"RARE"
		}
	}
}
impl TryFrom<&str> for Rarity
{
	type Error = ();
	fn try_from(i: &str) -> Result<Self, Self::Error>
	{
		match i
		{
			"COMMON"=>Ok(Self::COMMON),
			"UNCOMMON"=>Ok(Self::UNCOMMON),
			"RARE"=>Ok(Self::RARE),
			_=>Err(())
		}
	}
}

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
    type Error = ();

    fn try_from(value: RelicArcane) -> Result<Self, Self::Error> {
        if let Some(relic_rewards) = value.relic_rewards
		{
			Ok(Self{unique_name: value.unique_name, name: value.name, relic_rewards})
		} else {Err(())}
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reward
{
	pub reward_name: String,
	pub rarity: Rarity,
}

pub(crate) fn load(cache: &Path, manifest: &str) -> anyhow::Result<Vec<Relic>>
{
	let file = load::load(cache, manifest)?;
	let parsed: Export = serde_json::from_str(&file)?;
	let relics = parsed.export_relic_arcane.into_iter()
		.flat_map(|r|r.try_into())
		.collect();
	Ok(relics)
}