use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use anyhow::Result;

mod invasions;
mod types;
mod recipes;
mod requires;
mod relics;
mod relic_rewards;
mod resources;
mod active_relics;
mod resurgence_relics;

use crate::cache;
use crate::relic::Relic;
use active_relics::ActiveRelics;
use invasions::Invasions;
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
	invasions: Invasions,
	recipes: Recipes,
	relics: Relics,
	relic_rewards: RelicRewards,
	requires: Requires,
	resources: Resources,
	resurgence_relics: ResurgenceRelics,
}

struct Interner<K, V>(HashMap<K, V>);
impl <K, V>Interner<K, V>
{
	fn new() -> Self
	{
		Self(HashMap::new())
	}
}

impl <K, V>Interner<K, V>
	where
		K: Clone + Hash + Eq,
		V: From<K> + Clone
{
	fn intern(&mut self, k: K) -> V
	{
		self.0
			.entry(k.clone())
			.or_insert(k.into())
			.to_owned()
	}
}

impl Data
{
	pub fn from_cache(cache_dir: &Path) -> Result<Self>
	{
		let mut common_names: Interner<String, CommonName> = Interner::new();
		let mut unique_names: Interner<String, UniqueName> = Interner::new();
		let index = cache::load_index(&cache_dir.join("index_en.txt.lzma"))?;

		let mut resources = Resources::default();
		for resource in cache::load_resources(cache_dir, &index["ExportResources_en.json"])?
		{
			let common_name = common_names.intern(resource.name);
			let unique_name = unique_names.intern(resource.unique_name);
			resources.add(unique_name, common_name);
		}

		for warframe in cache::load_warframes(cache_dir, &index["ExportWarframes_en.json"])?
		{
			let unique_name = unique_names.intern(warframe.unique_name);
			let common_name = warframe.name
				.strip_prefix("<ARCHWING> ")
				.unwrap_or(&warframe.name)
				.to_owned();
			let common_name = common_names.intern(common_name);
			resources.add(unique_name, common_name);
		}

		for weapon in cache::load_weapons(cache_dir, &index["ExportWeapons_en.json"])?
		{
			let common_name = common_names.intern(weapon.name);
			let unique_name = unique_names.intern(weapon.unique_name);
			resources.add(unique_name, common_name);
		}

		let mut requires = Requires::default();
		let mut recipes = Recipes::default();
		for recipe in cache::load_recipes(cache_dir, &index["ExportRecipes_en.json"])?
		{
			let recipe_unique_name = unique_names.intern(recipe.unique_name);
			let recipe_result_type = unique_names.intern(recipe.result_type);
			recipes.add(recipe_unique_name.clone(), recipe_result_type);
			for ingredient in recipe.ingredients
			{
				let ingredient_item_type = unique_names.intern(ingredient.item_type);
				requires.add(
					recipe_unique_name.clone(),
					ingredient_item_type,
					ingredient.item_count);
			}
		}

		let mut relics = Relics::default();
		let mut relic_rewards = RelicRewards::default();
		for relic in cache::load_relics(cache_dir, &index["ExportRelicArcane_en.json"])?
		{
			let relic_unique_name = unique_names.intern(relic.unique_name);
			let relic_common_name = common_names.intern(relic.name);
			relics.add(relic_unique_name.clone(), relic_common_name);
			for reward in relic.relic_rewards
			{
				let reward_unique_name = reward.reward_name
					.split('/')
					.filter(|&s|s != "StoreItems")
					.fold(String::new(), |a, b|a+b+"/");
				let reward_unique_name = reward_unique_name
					.strip_suffix('/')
					.unwrap_or(&reward_unique_name)
					.to_owned();
				let reward_unique_name = unique_names.intern(reward_unique_name);
				relic_rewards.add(
					relic_unique_name.clone(),
					reward_unique_name,
					reward.rarity);
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

		let mut invasions = Invasions::default();
		for invasion in cache::invasions(&cache_dir.join("worldstate.json"))?
		{
			invasions.add(invasion.into());
		}

		Ok(Self
		{
			recipes,
			relics,
			requires,
			resources,
			invasions,
			active_relics,
			resurgence_relics,
			relic_rewards,
		})
	}

	pub fn requirements(&self, recipe_unique_name: UniqueName) -> impl Iterator<Item = (UniqueName, Count)> + '_
	{
		self.requires.fetch_by_recipe_unique_name(recipe_unique_name)
	}

	pub fn resource_common_name(&self, unique_name: UniqueName) -> Option<CommonName>
	{
		self.resources.fetch_by_unique_name(unique_name)
	}

	pub fn resource_unique_name(&self, common_name: impl Into<CommonName>) -> Option<UniqueName>
	{
		self.resources.fetch_by_common_name(common_name.into())
	}

	pub fn _how_many_needed(
		&self,
		recipe_unique_name: UniqueName,
		resource_unique_name: UniqueName) -> Option<Count>
	{
		self.requires.fetch_by_recipe_unique_name(recipe_unique_name)
			.find(|(item, _)| *item == resource_unique_name)
			.map(|(_item, count)|count)
	}

	pub fn active_relics(&self, component_unique_name: UniqueName) -> Option<Vec<Relic>>
	{
		let relic_rewards = self.relic_rewards
			.fetch_by_reward_unique_name(component_unique_name);

		let mut relics = vec![];
		for (relic_unique_name, reward_rarity) in relic_rewards
		{
			let relic_common_name = self.relics
				.fetch_by_unique_name(relic_unique_name)?;
			if self.active_relics.is_active(relic_common_name.clone())
			{
				let relic = Relic::new(relic_common_name, reward_rarity);
				relics.push(relic);
			}
		}
		relics.sort();
		relics.dedup();
		Some(relics)
	}

	pub fn resurgence_relics(&self, component_unique_name: UniqueName) -> Option<Vec<Relic>>
	{
		let relic_rewards = self.relic_rewards
			.fetch_by_reward_unique_name(component_unique_name);

		let mut relics = vec![];
		for (relic_unique_name, reward_rarity) in relic_rewards
		{
			let relic_common_name = self.relics
				.fetch_by_unique_name(relic_unique_name.clone())?;
			if self.resurgence_relics.is_active(relic_unique_name.clone())
			{
				let relic = Relic::new(relic_common_name, reward_rarity);
				relics.push(relic);
			}
		}
		relics.sort();
		relics.dedup();
		Some(relics)
	}

	pub fn recipes(&self, result_type: UniqueName) -> impl Iterator<Item = UniqueName> + '_
	{
		self.recipes.fetch_by_result_type(result_type)
	}

	pub fn recipe(&self, result_type: UniqueName) -> Option<UniqueName>
	{
		self.recipes(result_type).next()
	}

	pub fn recipe_result(&self, recipe_unique_name: UniqueName) -> Option<UniqueName>
	{
		self.recipes.fetch_by_unique_name(recipe_unique_name)
	}

	pub fn available_from_invasion(&self, unique_name: UniqueName) -> bool
	{
		self.invasions.drops_from_invasion(unique_name)
	}

}
