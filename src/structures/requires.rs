use std::collections::HashMap;

use anyhow::{anyhow, Result};

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
	pub fn fetch_by_recipe_unique_name(&self, recipe_unique_name: &str) -> Result<Vec<(UniqueName, Count)>>
	{
		let indexes = self.recipe_unique_name_index.get(&recipe_unique_name.into())
			.ok_or_else(||anyhow!("Value does not exist in unique_name index"))?;

		let mut rows = Vec::with_capacity(indexes.len());
		for index in indexes
		{
			let unique_name = self.item_types.get(*index)
				.ok_or_else(||anyhow!("Value does not exist"))?
				.clone();
			let count = self.count.get(*index)
				.ok_or_else(||anyhow!("Value does not exist"))?
				.clone();
			rows.push((unique_name, count));
		}
		Ok(rows)
	}

	pub fn fetch_by_item_type(
		&self,
		item_type: impl Into<UniqueName>) -> Result<(UniqueName, Count)>
	{
		let index = *self.item_type_index.get(&item_type.into())
			.ok_or_else(||anyhow!("Value does not exist in unique_name index"))?;
		let recipe_unique_name = self.recipe_unique_names.get(index)
			.ok_or_else(||anyhow!("Value does not exist"))?
			.clone();
		let count = self.count.get(index)
			.ok_or_else(||anyhow!("Value does not exist"))?
			.clone();
		Ok((recipe_unique_name, count))
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
			let _ = Requires::default();
		}

		#[test]
		fn get_nonexistent_recipe()
		{
			let r = Requires::default();
			assert!(r.fetch_by_item_type("foo").is_err());
		}

		#[test]
		fn get_nonexistent_item_type()
		{
			let r = Requires::default();
			assert!(r.fetch_by_recipe_unique_name("foo").is_err());
		}
	}

	mod one
	{
		use super::*;
		#[test]
		fn new()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
		}

		#[test]
		fn get_nonexistent_recipe()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			assert!(r.fetch_by_item_type("baz").is_err());
		}

		#[test]
		fn get_nonexistent_item_type()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			assert!(r.fetch_by_recipe_unique_name("quix").is_err());
		}

		#[test]
		fn get_recipe()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			let foo: UniqueName = "foo".into();
			let one: Count = 1.into();
			let res = r.fetch_by_item_type("bar").unwrap();
			assert_eq!(res, (foo, one));
		}

		#[test]
		fn get_item_type()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			let bar: UniqueName = "bar".into();
			let one: Count = 1.into();
			let res = r.fetch_by_recipe_unique_name("foo").unwrap();
			assert_eq!(res, vec![(bar, one)]);
		}
	}

	mod two
	{
		use super::*;
		#[test]
		fn new()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
		}

		#[test]
		fn get_nonexistent_recipe()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
			assert!(r.fetch_by_item_type("baz").is_err());
		}

		#[test]
		fn get_nonexistent_item_type()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
			assert!(r.fetch_by_recipe_unique_name("quix").is_err());
		}

		#[test]
		fn get_first_recipe()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
			let foo: UniqueName = "foo".into();
			let one: Count = 1.into();
			let res = r.fetch_by_item_type("bar").unwrap();
			assert_eq!(res, (foo, one));
		}

		#[test]
		fn get_first_item_type()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
			let bar: UniqueName = "bar".into();
			let one: Count = 1.into();
			let res = r.fetch_by_recipe_unique_name("foo").unwrap();
			assert_eq!(res, vec![(bar, one)]);
		}

		#[test]
		fn get_second_recipe()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
			let baz: UniqueName = "baz".into();
			let two: Count = 2.into();
			let res = r.fetch_by_item_type("quix").unwrap();
			assert_eq!(res, (baz, two));
		}

		#[test]
		fn get_second_item_type()
		{
			let mut r = Requires::default();
			r.add("foo", "bar", 1);
			r.add("baz", "quix", 2);
			let quix: UniqueName = "quix".into();
			let two: Count = 2.into();
			let res = r.fetch_by_recipe_unique_name("baz").unwrap();
			assert_eq!(res, vec![(quix, two)]);
		}
	}
}