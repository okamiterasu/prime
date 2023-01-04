use std::collections::HashMap;

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
		unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, RelicRewardRarity)> + '_
	{
		let indices = self.relic_index.get(&unique_name)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|Some((self.rewards.get(index)?, self.rarities.get(index)?)))
			.map(|(reward, rarity)|(reward.clone(), rarity.clone()))
	}

	pub fn fetch_by_reward_unique_name(
		&self,
		unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, RelicRewardRarity)> + '_
	{
		let indices = self.reward_index.get(&unique_name)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|Some((self.relics.get(index)?, self.rarities.get(index)?)))
			.map(|(relic, rarity)|(relic.clone(), rarity.clone()))
	}

	pub fn add(
		&mut self,
		relic_unique_name: impl Into<UniqueName>,
		reward_unique_name: impl Into<UniqueName>,
		reward_rarity: RelicRewardRarity)
	{
		let index = self.relics.len();
		let relic_unique_name = relic_unique_name.into();
		self.relics.push(relic_unique_name.clone());
		self.relic_index.entry(relic_unique_name)
			.or_default()
			.push(index);

		let reward_unique_name = reward_unique_name.into();
		self.rewards.push(reward_unique_name.clone());
		self.reward_index.entry(reward_unique_name)
			.or_default()
			.push(index);

		self.rarities.push(reward_rarity);
	}
}