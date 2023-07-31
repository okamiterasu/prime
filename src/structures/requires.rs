use std::collections::HashMap;

use super::types::{Count, UniqueName};

/// Recipe, Item Type, Count
type Row = (UniqueName, UniqueName, Count);

#[derive(Default, Debug)]
pub struct Requires
{
	rows: Vec<Row>,
	recipe_unique_name_index: HashMap<UniqueName, Vec<usize>>,
	item_type_index: HashMap<UniqueName, usize>,
}

impl Requires
{
	pub fn fetch_by_recipe_unique_name(
		&self,
		recipe_unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, Count)> + '_
	{
		let indices = self.recipe_unique_name_index.get(&recipe_unique_name)
			.map(Vec::as_slice)
			.unwrap_or_default();
		indices.iter()
			.flat_map(|&index|self.rows.get(index))
			.map(|row|(row.1.clone(), row.2))
	}

	pub fn _fetch_by_item_type(
		&self,
		item_type: UniqueName) -> Option<(UniqueName, Count)>
	{
		let &index = self.item_type_index.get(&item_type)?;
		self.rows.get(index)
			.map(|row|(row.0.clone(), row.2))
	}

	pub fn add(
		&mut self,
		recipe_unique_name: UniqueName,
		item_type: UniqueName,
		count: Count)
	{
		let index = self.rows.len();
		self.rows.push((recipe_unique_name.clone(), item_type.clone(), count));
		self.recipe_unique_name_index.entry(recipe_unique_name)
			.or_default()
			.push(index);
		self.item_type_index.insert(item_type, index);
	}
}