use std::collections::HashMap;

use super::types::UniqueName;

#[derive(Default, Debug)]
pub struct Recipes
{
	pub unique_names: Vec<UniqueName>,
	pub result_types: Vec<UniqueName>,
	pub unique_name_index: HashMap<UniqueName, usize>,
	pub result_type_index: HashMap<UniqueName, Vec<usize>>,
}

impl Recipes
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<UniqueName>
	{
		let &i = self.unique_name_index.get(&unique_name)?;
		self.result_types.get(i).cloned()
	}

	pub fn fetch_by_result_type(
		&self,
		result_type: UniqueName) -> impl Iterator<Item = UniqueName> + '_
	{
		let indices = self.result_type_index.get(&result_type)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|self.unique_names.get(index))
			.cloned()
	}

	pub fn add(
		&mut self,
		unique_name: impl Into<UniqueName>,
		result_type: impl Into<UniqueName>)
	{
		let unique_name = unique_name.into();
		let result_type = result_type.into();
		let index = self.unique_names.len();
		self.unique_names.push(unique_name.clone());
		self.unique_name_index.insert(unique_name, index);
		self.result_types.push(result_type.clone());
		self.result_type_index.entry(result_type)
			.or_default()
			.push(index);
	}
}