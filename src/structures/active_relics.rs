use std::collections::HashSet;

use super::types::CommonName;

#[derive(Default, Debug)]
pub struct ActiveRelics
{
	common_names: HashSet<CommonName>
}

impl ActiveRelics
{
	pub fn is_active(&self, common_name: CommonName) -> bool
	{
		self.common_names.contains(&common_name)
	}

	pub fn add(&mut self, common_name: CommonName)
	{
		self.common_names.insert(common_name);
	}
}