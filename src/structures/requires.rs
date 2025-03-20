use super::types::{Count, UniqueName};

/// Recipe, Item Type, Count
type Row = (UniqueName, UniqueName, Count);

#[derive(Default, Debug)]
pub struct Requires
{
	rows: Vec<Row>
}

impl Requires
{
	pub fn fetch_by_recipe_unique_name(
		&self,
		recipe_unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, Count)> + '_
	{
		self.rows
			.iter()
			.filter(move |&row|row.0.as_str() == recipe_unique_name.as_str())
			.map(|row|(row.1.clone(), row.2))
	}

	pub fn add(
		&mut self,
		recipe_unique_name: UniqueName,
		item_type: UniqueName,
		count: Count)
	{
		self.rows.push((recipe_unique_name.clone(), item_type.clone(), count));
	}
}