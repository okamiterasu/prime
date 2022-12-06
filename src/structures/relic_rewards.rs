use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::cache::RelicRewardRarity;

use super::types::UniqueName;

#[derive(Default, Debug)]
pub struct RelicRewards
{
	relics: Vec<UniqueName>,
	rarities: Vec<RelicRewardRarity>,
	rewards: Vec<UniqueName>,
	relic_index: HashMap<UniqueName, Vec<usize>>,
	reward_index: HashMap<UniqueName, Vec<usize>>,
}

impl RelicRewards
{
	pub fn _fetch_by_relic_unique_name(
		&self,
		unique_name: impl Into<UniqueName>) -> Result<Vec<(UniqueName, RelicRewardRarity)>>
	{
		let indices = self.relic_index.get(&unique_name.into())
			.ok_or_else(||anyhow!("Relic reward does not exist"))?
			.clone();
		let mut t = Vec::with_capacity(indices.len());
		for index in indices
		{
			let reward_unique_name = self.rewards.get(index)
				.ok_or_else(||anyhow!("Relic reward does not exist"))?
				.clone();
			let reward_rarity = self.rarities.get(index)
				.ok_or_else(||anyhow!("Relic reward does not exist"))?
				.clone();
			t.push((reward_unique_name, reward_rarity))
		}
		Ok(t)
	}

	pub fn fetch_by_reward_unique_name(
		&self,
		unique_name: UniqueName) -> Result<Vec<(UniqueName, RelicRewardRarity)>>
	{
		let un = unique_name;
		let indices = match self.reward_index.get(&un)
		{
			Some(i)=>i,
			None=>return Ok(vec![])
		};

		let mut t = Vec::with_capacity(indices.len());
		for index in indices
		{
			let relic_unique_name = self.relics.get(*index)
				.ok_or_else(||anyhow!("Recipe does not exist"))?
				.clone();
			let reward_rarity = self.rarities.get(*index)
				.ok_or_else(||anyhow!("Relic reward does not exist"))?
				.clone();
			t.push((relic_unique_name, reward_rarity))
		}
		Ok(t)
	}

	pub fn add(
		&mut self,
		relic_unique_name: impl Into<UniqueName>,
		reward_unique_name: impl Into<UniqueName>,
		reward_rarity: RelicRewardRarity)
	{
		let index = self.relics.len();
		let run = relic_unique_name.into();
		self.relics.push(run.clone());
		self.relic_index.entry(run)
			.or_default()
			.push(index);

		let run = reward_unique_name.into();
		self.rewards.push(run.clone());
		self.reward_index.entry(run)
			.or_default()
			.push(index);

		self.rarities.push(reward_rarity);
	}
}