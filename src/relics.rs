use std::path::{Path};
// use std::collections::HashMap;

use druid::Data;
use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Data, Hash)]
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

pub fn parse_from_file(path: &Path) -> std::io::Result<Vec<Relic>>
{
	let file_contents = std::fs::read_to_string(path)?;
	let escaped = file_contents
		.replace(r"\r", "")
		.replace(&['\r', '\n'][..], "");
	let parsed: Export = serde_json::from_str(&escaped)?;
	let relicarcanes = parsed.export_relic_arcane;
	let relics = relicarcanes.into_iter()
		.filter_map(|r|r.try_into().ok())
		.collect();
	Ok(relics)
}