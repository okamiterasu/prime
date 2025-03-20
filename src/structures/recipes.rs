use super::types::UniqueName;

/// (Unique Name, Result Type)
type Row = (UniqueName, UniqueName);

#[derive(Default, Debug)]
pub struct Recipes
{
	pub rows: Vec<Row>
}

impl Recipes
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<UniqueName>
	{
		let unique_name = unique_name.as_str();
		self.rows
			.iter()
			.filter(|row|row.0.as_str().eq_ignore_ascii_case(unique_name))
			.map(|row|&row.1)
			.next()
			.cloned()
	}

	pub fn fetch_by_result_type(
		&self,
		result_type: UniqueName) -> impl Iterator<Item = UniqueName> + '_
	{
		self.rows
			.iter()
			.filter(move |&row|row.1.as_str() == result_type.as_str())
			.map(|row|&row.0)
			.cloned()
	}

	pub fn add(
		&mut self,
		unique_name: UniqueName,
		result_type: UniqueName)
	{
		self.rows.push((unique_name.clone(), result_type.clone()));
	}
}