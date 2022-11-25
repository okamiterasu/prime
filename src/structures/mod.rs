use std::path::Path;

use anyhow::{anyhow, Result, Context};

mod items;
mod types;
mod recipes;
mod requires;
mod relics;
mod relic_rewards;
mod resources;
mod active_relics;
mod resurgence_relics;

use active_relics::ActiveRelics;
use crate::cache;
use items::Items;
use recipes::Recipes;
use relics::Relics;
use requires::Requires;
use relic_rewards::RelicRewards;
use resources::Resources;
use resurgence_relics::ResurgenceRelics;
pub use types::{UniqueName, Count, CommonName};

#[derive(Debug)]
pub struct Data
{
	active_relics: ActiveRelics,
	items: Items,
	recipes: Recipes,
	relics: Relics,
	relic_rewards: RelicRewards,
	requires: Requires,
	resources: Resources,
	resurgence_relics: ResurgenceRelics,
}

impl Data
{
	pub fn from_cache(cache_dir: &Path) -> Result<Self>
	{
		let index = cache::load_index(&cache_dir.join("index_en.txt"))?;

		let mut items = Items::default();
		for warframe in cache::load_warframes(cache_dir, &index["ExportWarframes_en.json"])?
		{
			let unique_name = warframe.unique_name.as_ref();
			let common_name = warframe.name.strip_prefix("<ARCHWING> ")
				.unwrap_or(&warframe.name);
			items.add(unique_name, common_name);
		}

		for weapon in cache::load_weapons(cache_dir, &index["ExportWeapons_en.json"])?
		{
			let unique_name = weapon.unique_name.as_ref();
			let common_name = weapon.name.as_ref();
			items.add(unique_name, common_name);
		}

		let mut requires = Requires::default();
		let mut recipes = Recipes::default();
		for recipe in cache::load_recipes(cache_dir, &index["ExportRecipes_en.json"])?
		{
			let recipe_unique_name = recipe.unique_name.as_ref();
			recipes.add(recipe_unique_name, recipe.result_type);
			for ingredient in recipe.ingredients
			{
				requires.add(
					recipe_unique_name,
					ingredient.item_type,
					ingredient.item_count);
			}
		}

		let mut relics = Relics::default();
		let mut relic_rewards = RelicRewards::default();
		for relic in cache::load_relics(cache_dir, &index["ExportRelicArcane_en.json"])?
		{
			let relic_unique_name = relic.unique_name;
			let relic_common_name = relic.name;
			relics.add(relic_unique_name.as_ref(), relic_common_name);
			for reward in relic.relic_rewards
			{
				let reward_unique_name = reward.reward_name
					.split('/')
					.filter(|s|*s != "StoreItems")
					.fold(String::new(), |a, b|a+b+"/");
				let reward_unique_name = reward_unique_name
					.strip_suffix('/')
					.unwrap_or(&reward_unique_name);
				relic_rewards.add(relic_unique_name.as_ref(), reward_unique_name, reward.rarity);
			}
		}

		let mut active_relics = ActiveRelics::default();
		for active_relic in cache::active_relics(&cache_dir.join("droptable.html"))?
		{
			active_relics.add(active_relic.into());
		}

		let mut resurgence_relics = ResurgenceRelics::default();
		for resurgence_relic in cache::resurgence_relics(&cache_dir.join("worldstate.json"))?
		{
			resurgence_relics.add(resurgence_relic.into());
		}

		let mut resources = Resources::default();
		for resource in cache::load_resources(cache_dir, &index["ExportResources_en.json"])?
		{
			resources.add(resource.unique_name, resource.name);
		}

		Ok(Self
		{
			recipes,
			relics,
			requires,
			resources,
			items,
			active_relics,
			resurgence_relics,
			relic_rewards,
		})
	}

	pub fn requirements(&self, recipe_unique_name: impl Into<UniqueName>) -> Result<Vec<(UniqueName, Count)>>
	{
		self.requires.fetch_by_recipe_unique_name(recipe_unique_name.into())
	}

	pub fn item_common_name(&mut self, unique_name: impl Into<UniqueName>) -> Result<Option<CommonName>>
	{
		self.items.fetch_by_unique_name(unique_name.into())
	}

	pub fn item_unique_name(&mut self, common_name: impl Into<CommonName>) -> Result<UniqueName>
	{
		self.items.fetch_by_common_name(common_name.into())
	}

	pub fn resource_common_name(&mut self, unique_name: impl Into<UniqueName>) -> Result<Option<CommonName>>
	{
		self.resources.fetch_by_unique_name(unique_name.into())
	}

	pub fn how_many_needed(&mut self, recipe_unique_name: impl Into<UniqueName>, resource_unique_name: impl Into<UniqueName>) -> Result<Count>
	{
		let rec = recipe_unique_name.into();
		let res = resource_unique_name.into();
		let requires = self.requires.fetch_by_recipe_unique_name(rec)?;
		let (_item, count) = requires.into_iter().find(|i|i.0 == res)
			.ok_or_else(||anyhow!("Requirement does not exist"))?;
		Ok(count)
	}

	pub fn active_component_relics(&self, component_unique_name: impl Into<UniqueName>) -> Result<Vec<crate::Relic>>
	{
		let relic_rewards = self.relic_rewards
			.fetch_by_reward_unique_name(component_unique_name)?;

		let mut relics = Vec::with_capacity(relic_rewards.len());
		for (relic_unique_name, reward_rarity) in relic_rewards
		{
			let relic_common_name = self.relics
				.fetch_by_unique_name(relic_unique_name.as_str())?;
			if self.active_relics.is_active(relic_common_name.clone())
			{
				let relic = crate::Relic::new(
					relic_common_name.as_str(),
					reward_rarity.as_str())?;
				relics.push(relic);
			}
		}
		relics.sort();
		relics.dedup();
		Ok(relics)
	}

	pub fn active_recipe_relics(&self, recipe_unique_name: impl Into<UniqueName>) -> Result<Vec<crate::Relic>>
	{
		let recipe = self.recipes.fetch_by_unique_name(recipe_unique_name)
			.context("Looking for recipes by unique_name")?;
		let relic_rewards = self.relic_rewards.fetch_by_reward_unique_name(recipe)
			.context("Looking for relic rewards needed by recipe")?;
		let mut relics = Vec::with_capacity(relic_rewards.len());
		for (relic_unique_name, reward_rarity) in relic_rewards
		{
			let relic_common_name = self.relics
				.fetch_by_unique_name(relic_unique_name.as_str())?;
			if self.active_relics.is_active(relic_common_name.clone())
			{
				let relic = crate::Relic::new(
					relic_common_name.as_str(),
					reward_rarity.as_str())?;
				relics.push(relic);
			}
		}
		relics.sort();
		relics.dedup();
		Ok(relics)
	}

	pub fn resurgence_component_relics(&self, component_unique_name: impl Into<UniqueName>) -> Result<Vec<crate::Relic>>
	{
		let relic_rewards = self.relic_rewards
			.fetch_by_reward_unique_name(component_unique_name)?;

		let mut relics = Vec::with_capacity(relic_rewards.len());
		for (relic_unique_name, reward_rarity) in relic_rewards
		{
			let relic_common_name = self.relics
				.fetch_by_unique_name(relic_unique_name.as_str())?;
			if self.resurgence_relics.is_active(relic_common_name.clone())
			{
				let relic = crate::Relic::new(
					relic_common_name.as_str(),
					reward_rarity.as_str())?;
				relics.push(relic);
			}
		}
		relics.sort();
		relics.dedup();
		Ok(relics)
	}

	pub fn resurgence_recipe_relics(&self, recipe_unique_name: impl Into<UniqueName>) -> Result<Vec<crate::Relic>>
	{
		let recipe = self.recipes.fetch_by_unique_name(recipe_unique_name)?;
		let relic_rewards = self.relic_rewards.fetch_by_reward_unique_name(recipe)?;
		let mut relics = Vec::with_capacity(relic_rewards.len());
		for (relic_unique_name, reward_rarity) in relic_rewards
		{
			let relic_common_name = self.relics
				.fetch_by_unique_name(relic_unique_name.as_str())?;
			if self.resurgence_relics.is_active(relic_common_name.clone())
			{
				let relic = crate::Relic::new(
					relic_common_name.as_str(),
					reward_rarity.as_str())?;
				relics.push(relic);
			}
		}
		relics.sort();
		relics.dedup();
		Ok(relics)
	}

	pub fn recipes(&self, result_type: impl Into<UniqueName>) -> Result<Vec<UniqueName>>
	{
		let recipes = self.recipes.fetch_by_result_type(result_type)?;
		let mut t = Vec::with_capacity(recipes.len());
		for recipe in recipes
		{
			t.push(recipe);
		}
		Ok(t)
	}

	pub fn recipe(&self, result_type: impl Into<UniqueName>) -> Result<Option<UniqueName>>
	{
		Ok(self.recipes(result_type)?.pop())
	}
}