use super::types::{UniqueName, CommonName};

type Row = (UniqueName, CommonName);

#[derive(Default, Debug)]
pub struct Relics
{
	rows: Vec<Row>
}

impl Relics
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<CommonName>
	{
		self.rows
			.iter()
			.filter(|row|row.0.as_str().eq_ignore_ascii_case(unique_name.as_str()))
			.map(|row|&row.1)
			.next()
			.cloned()
	}

	pub fn add(
		&mut self,
		unique_name: UniqueName,
		common_name: CommonName)
	{
		self.rows.push((unique_name.clone(), common_name.clone()));
	}
}