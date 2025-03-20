use crate::relic::Rarity;
use super::types::UniqueName;

/// (Relic, Reward, Rarity)
type Row = (UniqueName, UniqueName, Rarity);

#[derive(Default, Debug)]
pub struct RelicRewards
{
	rows: Vec<Row>
}

impl RelicRewards
{
	pub fn fetch_by_reward_unique_name(
		&self,
		unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, Rarity)> + '_
	{
		self.rows
			.iter()
			.filter(move |&row|row.1.as_str() == unique_name.as_str())
			.map(|row|(row.0.clone(), row.2))
	}

	pub fn add(
		&mut self,
		relic_unique_name: UniqueName,
		reward_unique_name: UniqueName,
		reward_rarity: Rarity)
	{
		self.rows.push((
			relic_unique_name.clone(),
			reward_unique_name.clone(),
			reward_rarity));
	}
}