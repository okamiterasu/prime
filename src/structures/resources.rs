use super::types::{UniqueName, CommonName};


type Row = (UniqueName, CommonName);

#[derive(Default, Debug)]
pub struct Resources
{
	rows: Vec<Row>
}

impl Resources
{
	pub fn fetch_by_unique_name(
		&self,
		unique_name: UniqueName) -> Option<CommonName>
	{
		let unique_name = unique_name.as_str();
		self.rows
			.iter()
			.filter(|row|row.0.as_str().eq_ignore_ascii_case(unique_name))
			.map(|row|&row.1)
			.next()
			.cloned()
	}

	pub fn fetch_by_common_name(
		&self,
		common_name: CommonName) -> Option<UniqueName>
	{
		let common_name = common_name.as_str();
		self.rows
			.iter()
			.filter(|row|row.1.as_str().eq_ignore_ascii_case(common_name))
			.map(|row|&row.0)
			.nth(0)
			.cloned()
	}

	pub fn add(&mut self, unique_name: UniqueName, common_name: CommonName)
	{
		self.rows.push((unique_name.clone(), common_name.clone()));
	}
}