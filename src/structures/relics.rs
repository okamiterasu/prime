use std::collections::HashMap;

use super::types::{UniqueName, CommonName};

type Row = (UniqueName, CommonName);

#[derive(Default, Debug)]
pub struct Relics
{
	rows: Vec<Row>,
	unique_name_index: HashMap<UniqueName, usize>,
	common_name_index: HashMap<CommonName, usize>,
}

impl Relics
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<CommonName>
	{
		let &index = self.unique_name_index.get(&unique_name)?;
		self.rows.get(index)
			.map(|r|&r.1)
			.cloned()
	}

	pub fn _fetch_by_common_name(
		&self,
		common_name: CommonName) -> Option<UniqueName>
	{
		let &index = self.common_name_index.get(&common_name)?;
		self.rows.get(index)
			.map(|r|&r.0)
			.cloned()
	}

	pub fn add(
		&mut self,
		unique_name: UniqueName,
		common_name: CommonName)
	{
		let index = self.rows.len();
		self.rows.push((unique_name.clone(), common_name.clone()));
		self.unique_name_index.insert(unique_name, index);
		self.common_name_index.insert(common_name, index);
	}
}