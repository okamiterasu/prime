use std::collections::HashSet;

use super::types::UniqueName;

#[derive(Default, Debug)]
pub struct Invasions
{
	unique_name: HashSet<UniqueName>
}

impl Invasions
{
	pub fn drops_from_invasion(&self, unique_name: UniqueName) -> bool
	{
		self.unique_name.contains(&unique_name)
	}

	pub fn add(&mut self, common_name: UniqueName)
	{
		self.unique_name.insert(common_name);
	}
}