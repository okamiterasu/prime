use anyhow::{Result, Context};

use crate::item_view::ItemView;
use crate::structures::{CommonName, UniqueName, Data};
use crate::relic::Relic;

#[derive(Debug)]
pub struct Recipe
{
	common_name: CommonName,
	unique_name: UniqueName,
	recipe_type: RecipeType
}

#[derive(Debug)]
pub enum RecipeType
{
	Normal(NormalRecipe),
	Prime(PrimeRecipe)
}

impl Recipe
{
	/// Create a recipe using the recipe's unique name
	pub fn new(db: &Data, unique_name: UniqueName) -> Result<Self>
	{
		let result_unique_name = db.recipe_result(unique_name.clone())
			.context("Looking for recipe result")?;
		let common_name = db.resource_common_name(result_unique_name)
			.context("Looking for recipe result common name")?;
		let common_name = format!("{common_name} Blueprint").into();
		Recipe::with_common_name(db, unique_name, common_name)
	}

	pub fn with_common_name(db: &Data, unique_name: UniqueName, common_name: CommonName) -> Result<Self>
	{
		let recipe_type = if common_name.as_str().contains("Prime")
		{
			let active_relics = db.active_relics(unique_name.clone())
				.unwrap_or_default();
			let resurgence_relics = db.resurgence_relics(unique_name.clone())
				.unwrap_or_default();
			let recipe = PrimeRecipe
			{
				active_relics,
				resurgence_relics
			};
			RecipeType::Prime(recipe)
		}
		else
		{
			let available_from_invasion = db.available_from_invasion(unique_name.clone());
			let recipe = NormalRecipe
			{
				available_from_invasion
			};
			RecipeType::Normal(recipe)
		};
		Ok(Recipe{common_name, unique_name, recipe_type})
	}
}

impl ItemView for Recipe
{
	fn common_name(&self) -> CommonName
	{
		self.common_name.clone()
	}

	fn unique_name(&self) -> UniqueName
	{
		self.unique_name.clone()
	}

	fn resurgence_relics(&self) -> &[Relic]
	{
		if let RecipeType::Prime(pr) = &self.recipe_type
		{
			&pr.resurgence_relics
		}
		else
		{
			&[]
		}
	}

	fn active_relics(&self) -> &[Relic]
	{
		if let RecipeType::Prime(pr) = &self.recipe_type
		{
			&pr.active_relics
		}
		else
		{
			&[]
		}
	}

	fn available_from_invasion(&self) -> bool
	{
		if let RecipeType::Normal(pr) = &self.recipe_type
		{
			pr.available_from_invasion
		}
		else
		{
			false
		}
	}
}

impl ItemView for &Recipe
{
	fn common_name(&self) -> CommonName
	{
		(*self).common_name()
	}

	fn unique_name(&self) -> UniqueName
	{
		(*self).unique_name()
	}

	fn resurgence_relics(&self) -> &[Relic]
	{
		(*self).resurgence_relics()
	}

	fn active_relics(&self) -> &[Relic]
	{
		(*self).active_relics()
	}

	fn available_from_invasion(&self) -> bool
	{
		(*self).available_from_invasion()
	}
}

#[derive(Debug)]
pub struct PrimeRecipe
{
	active_relics: Vec<Relic>,
	resurgence_relics: Vec<Relic>,
}

#[derive(Debug)]
pub struct NormalRecipe
{
	pub available_from_invasion: bool
}