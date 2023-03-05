use std::collections::HashMap;

use crate::relic::Rarity;
use super::types::UniqueName;

/// (Relic, Reward, Rarity)
type Row = (UniqueName, UniqueName, Rarity);

#[derive(Default, Debug)]
pub struct RelicRewards
{
	rows: Vec<Row>,
	relic_index: HashMap<UniqueName, Vec<usize>>,
	reward_index: HashMap<UniqueName, Vec<usize>>,
}

impl RelicRewards
{
	pub fn _fetch_by_relic_unique_name(
		&self,
		unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, Rarity)> + '_
	{
		let indices = self.relic_index.get(&unique_name)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|self.rows.get(index))
			.map(|row|(row.1.clone(), row.2))
	}

	pub fn fetch_by_reward_unique_name(
		&self,
		unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, Rarity)> + '_
	{
		let indices = self.reward_index.get(&unique_name)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|self.rows.get(index))
			.map(|row|(row.0.clone(), row.2))
	}

	pub fn add(
		&mut self,
		relic_unique_name: UniqueName,
		reward_unique_name: UniqueName,
		reward_rarity: Rarity)
	{
		let index = self.rows.len();

		self.rows.push((
			relic_unique_name.clone(),
			reward_unique_name.clone(),
			reward_rarity));

		self.relic_index.entry(relic_unique_name)
			.or_default()
			.push(index);

		self.reward_index.entry(reward_unique_name)
			.or_default()
			.push(index);
	}
}