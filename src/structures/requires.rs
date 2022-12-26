use std::collections::HashMap;

use super::types::{Count, UniqueName};

#[derive(Default, Debug)]
pub struct Requires
{
	recipe_unique_names: Vec<UniqueName>,
	item_types: Vec<UniqueName>,
	count: Vec<Count>,
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
			.flat_map(|&index|Some((self.item_types.get(index)?, self.count.get(index)?)))
			.map(|(un, c)|(un.clone(), c.clone()))
	}

	pub fn _fetch_by_item_type(
		&self,
		item_type: UniqueName) -> Option<(UniqueName, Count)>
	{
		let &index = self.item_type_index.get(&item_type)?;
		let recipe_unique_name = self.recipe_unique_names.get(index).cloned()?;
		let count = self.count.get(index).cloned()?;
		Some((recipe_unique_name, count))
	}

	pub fn add(
		&mut self,
		recipe_unique_name: impl Into<UniqueName>,
		item_type: impl Into<UniqueName>,
		count: impl Into<Count>)
	{
		let index = self.recipe_unique_names.len();
		let run = recipe_unique_name.into();
		self.recipe_unique_names.push(run.clone());
		self.recipe_unique_name_index.entry(run)
			.or_default()
			.push(index);

		let it = item_type.into();
		self.item_types.push(it.clone());
		self.item_type_index.insert(it, index);

		let c = count.into();
		self.count.push(c);
	}
}