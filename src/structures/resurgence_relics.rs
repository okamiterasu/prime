use std::collections::HashSet;

use super::types::UniqueName;

#[derive(Default, Debug)]
pub struct ResurgenceRelics
{
	unique_name: HashSet<UniqueName>
}

impl ResurgenceRelics
{
	pub fn is_active(&self, unique_name: UniqueName) -> bool
	{
		self.unique_name.contains(&unique_name)
	}

	pub fn add(&mut self, unique_name: UniqueName)
	{
		self.unique_name.insert(unique_name);
	}
}