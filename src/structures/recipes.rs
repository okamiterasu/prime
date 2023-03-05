use std::collections::HashMap;

use super::types::UniqueName;

/// (Unique Name, Result Type)
type Row = (UniqueName, UniqueName);

#[derive(Default, Debug)]
pub struct Recipes
{
	pub rows: Vec<Row>,
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
		self.rows.get(i).map(|r|&r.1).cloned()
	}

	pub fn fetch_by_result_type(
		&self,
		result_type: UniqueName) -> impl Iterator<Item = UniqueName> + '_
	{
		let indices = self.result_type_index.get(&result_type)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|self.rows.get(index))
			.map(|r|&r.0)
			.cloned()
	}

	pub fn add(
		&mut self,
		unique_name: UniqueName,
		result_type: UniqueName)
	{
		let index = self.rows.len();
		self.rows.push((unique_name.clone(), result_type.clone()));
		self.unique_name_index.insert(unique_name, index);
		self.result_type_index.entry(result_type)
			.or_default()
			.push(index);
	}
}