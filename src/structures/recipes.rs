use std::collections::HashMap;

use anyhow::{anyhow, Result};

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
		unique_name: impl Into<UniqueName>) -> Result<UniqueName>
	{
		let i = *self.unique_name_index.get(&unique_name.into())
			.ok_or_else(||anyhow!("Recipe does not exist in unique_names"))?;
		self.result_types.get(i)
			.cloned()
			.ok_or_else(||anyhow!("Recipe does not exist"))
	}

	pub fn fetch_by_result_type(
		&self,
		result_type: impl Into<UniqueName>) -> Result<Vec<UniqueName>>
	{
		let result_type = result_type.into();
		let indices = match self.result_type_index.get(&result_type)
		{
			Some(i)=>i,
			None=>return Ok(vec![])
		};
		let mut results = Vec::with_capacity(indices.len());
		for index in indices
		{
			let unique_name = self.unique_names.get(*index)
				.ok_or_else(||anyhow!("Recipe does not exist"))
				.cloned()?;
			results.push(unique_name);

		}
		Ok(results)
	}

	pub fn add(
		&mut self,
		unique_name: impl Into<UniqueName>,
		result_type: impl Into<UniqueName>)
	{
		let un = unique_name.into();
		let rt = result_type.into();
		let index = self.unique_names.len();
		self.unique_names.push(un.clone());
		self.unique_name_index.insert(un, index);
		self.result_types.push(rt.clone());
		self.result_type_index.entry(rt)
			.or_default()
			.push(index);
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	mod empty
	{
		use super::*;
		#[test]
		fn new()
		{
			let _ = Recipes::default();
		}

		#[test]
		fn get_nonexistent_name()
		{
			let r = Recipes::default();
			assert!(r.fetch_by_unique_name("foo").is_err());
		}

		#[test]
		fn get_nonexistent_result()
		{
			let r = Recipes::default();
			assert!(r.fetch_by_result_type("foo").is_err());
		}
	}

	mod one
	{
		use super::*;
		#[test]
		fn new()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
		}

		#[test]
		fn get_nonexistent_name()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			assert!(r.fetch_by_unique_name("baz").is_err());
		}

		#[test]
		fn get_nonexistent_item_type()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			assert!(r.fetch_by_result_type("quix").is_err());
		}

		#[test]
		fn get_name()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			let bar: UniqueName = "bar".into();
			let res = r.fetch_by_unique_name("foo").unwrap();
			assert_eq!(res, bar);
		}

		#[test]
		fn get_type()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			let foo: UniqueName = "foo".into();
			let res = r.fetch_by_result_type("bar").unwrap();
			assert_eq!(res, vec![foo]);
		}
	}

	mod two
	{
		use super::*;
		#[test]
		fn new()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
		}

		#[test]
		fn get_nonexistent_recipe()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
			assert!(r.fetch_by_result_type("baz").is_err());
		}

		#[test]
		fn get_nonexistent_item_type()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
			assert!(r.fetch_by_unique_name("quix").is_err());
		}

		#[test]
		fn get_first_recipe()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
			let foo: UniqueName = "foo".into();
			let res = r.fetch_by_result_type("bar").unwrap();
			assert_eq!(res, vec![foo]);
		}

		#[test]
		fn get_first_item_type()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
			let bar: UniqueName = "bar".into();
			let res = r.fetch_by_unique_name("foo").unwrap();
			assert_eq!(res, bar);
		}

		#[test]
		fn get_second_recipe()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
			let baz: UniqueName = "baz".into();
			let res = r.fetch_by_result_type("quix").unwrap();
			assert_eq!(res, vec![baz]);
		}

		#[test]
		fn get_second_item_type()
		{
			let mut r = Recipes::default();
			r.add("foo", "bar");
			r.add("baz", "quix");
			let quix: UniqueName = "quix".into();
			let res = r.fetch_by_unique_name("baz").unwrap();
			assert_eq!(res, quix);
		}
	}
}