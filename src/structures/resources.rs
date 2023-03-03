use std::collections::HashMap;

use super::types::{UniqueName, CommonName};

#[derive(Default, Debug)]
pub struct Resources
{
	unique_names: Vec<UniqueName>,
	common_names: Vec<CommonName>,
	unique_name_index: HashMap<UniqueName, usize>,
	common_name_index: HashMap<CommonName, usize>,
}

impl Resources
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<CommonName>
	{
		let &index = self.unique_name_index.get(&unique_name)?;
		self.common_names.get(index).cloned()
	}

	pub fn fetch_by_common_name(
		&self,
		common_name: CommonName) -> Option<UniqueName>
	{
		let &index = self.common_name_index.get(&common_name)?;
		self.unique_names.get(index).cloned()
	}

	pub fn add(&mut self, unique_name: UniqueName, common_name: CommonName)
	{
		let index = self.unique_names.len();
		self.unique_names.push(unique_name.clone());
		self.unique_name_index.insert(unique_name, index);

		self.common_names.push(common_name.clone());
		self.common_name_index.insert(common_name, index);
	}
}