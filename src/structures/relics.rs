use std::collections::HashMap;

use super::types::{UniqueName, CommonName};

#[derive(Default, Debug)]
pub struct Relics
{
	unique_names: Vec<UniqueName>,
	common_names: Vec<CommonName>,
	unique_name_index: HashMap<UniqueName, usize>,
	common_name_index: HashMap<CommonName, usize>,
}

impl Relics
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<CommonName>
	{
		let index = *self.unique_name_index.get(&unique_name)?;
		self.common_names.get(index).cloned()
	}

	pub fn _fetch_by_common_name(
		&self,
		common_name: CommonName) -> Option<UniqueName>
	{
		let index = *self.common_name_index.get(&common_name)?;
		self.unique_names.get(index).cloned()
	}

	pub fn add(
		&mut self,
		unique_name: impl Into<UniqueName>,
		common_name: impl Into<CommonName>)
	{
		let index = self.unique_names.len();
		let un = unique_name.into();
		self.unique_names.push(un.clone());
		self.unique_name_index.insert(un, index);

		let cn = common_name.into();
		self.common_names.push(cn.clone());
		self.common_name_index.insert(cn, index);
	}
}