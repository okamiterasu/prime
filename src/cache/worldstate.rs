use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct State
{
	invasions: Vec<Invastion>,
	prime_vault_traders: Vec<PrimeVaultTrader>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Invastion
{
	_faction: InvasionFaction,
	attacker_reward: RewardOrEmptyVec,
	defender_reward: RewardOrEmptyVec
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RewardOrEmptyVec
{
	Reward(Reward),
	EmptyVec(Vec<()>)
}

impl RewardOrEmptyVec
{
	fn into_reward(self) -> Reward
	{
		match self
		{
			Self::Reward(r)=>r,
			Self::EmptyVec(_)=>Reward::default()
		}
	}
}

#[derive(Deserialize, Debug)]
enum InvasionFaction
{
	#[serde(rename = "FC_GRINEER")]
	Grineer,
	#[serde(rename = "FC_CORPUS")]
	Corpus,
	#[serde(rename = "FC_INFESTATION")]
	Infested
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Reward
{
	counted_items: Vec<CountedItem>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct CountedItem
{
	item_type: String,
	_item_count: usize
}

pub fn invasions(file_path: &Path) -> Result<Vec<String>>
{
	if !file_path.exists()
	{
		let worldstate = crate::live::worldstate()?;
		std::fs::write(file_path, worldstate)?;
	}

	let reader = BufReader::new(File::open(file_path)?);
	let world_state: State = serde_json::from_reader(reader)?;
	let faction_rewards = world_state.invasions.into_iter()
		.flat_map(|invasion|
		{
			let attacker_rewards = invasion.attacker_reward.into_reward()
				.counted_items;
			let defender_rewards = invasion.defender_reward.into_reward()
				.counted_items;
			attacker_rewards.into_iter().chain(defender_rewards)
		});
	let rewards = faction_rewards.map(|i|i.item_type).collect();
	Ok(rewards)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PrimeVaultTrader
{
	pub manifest: Vec<Item>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item
{
	pub item_type: String,
}

pub fn resurgence_relics(file_path: &Path) -> Result<Vec<String>>
{
	if !file_path.exists()
	{
		let worldstate = crate::live::worldstate()?;
		std::fs::write(file_path, worldstate)?;
	}
	let reader = File::open(file_path)
		.map(BufReader::new)?;
	let world_state: State = serde_json::from_reader(reader)?;
	let store_items = world_state.prime_vault_traders.iter()
		.flat_map(|t|&t.manifest)
		.map(|i|&i.item_type[..])
		.filter(|i|i.starts_with("/Lotus/StoreItems/Types/Game/Projections/"));
	// Resurgence store items contain a "StoreItems" node in the path which
	// needs to be removed to make it line up with the rest of the names.
	// TODO: I feel like there should be a better way to do this.
	let normalized_store_items = store_items
		.map(|i|i.split('/').filter(|&s|s != "StoreItems").collect());
	let relics = normalized_store_items
		.map(|i: Vec<_>|i.join("/"))
		.collect();
	Ok(relics)
}