use std::collections::HashMap;

use anyhow::{anyhow, Result};

use super::types::{UniqueName, CommonName};

#[derive(Default, Debug)]
pub struct Items
{
	unique_names: Vec<UniqueName>,
	common_names: Vec<CommonName>,
	unique_name_index: HashMap<UniqueName, usize>,
	common_name_index: HashMap<CommonName, usize>,
}

impl Items
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: impl Into<UniqueName>+Clone) -> Result<Option<CommonName>>
	{
		match self.unique_name_index.get(&unique_name.into())
		{
			Some(index)=>
			{
				let common_name = self.common_names.get(*index)
					.ok_or_else(||anyhow!("Recipe does not exist"))?
					.clone();
				Ok(Some(common_name))
			},
			None=>Ok(None)
		}
	}

	pub fn fetch_by_common_name(
		&self,
		common_name: impl Into<CommonName>) -> Result<UniqueName>
	{
		let index = *self.common_name_index.get(&common_name.into())
			.ok_or_else(||anyhow!("Recipe does not exist in unique_names"))?;
		self.unique_names.get(index)
			.ok_or_else(||anyhow!("Recipe does not exist"))
			.map(|cn|cn.clone())
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