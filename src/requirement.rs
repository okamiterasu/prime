use anyhow::{Result, Context};

use crate::structures::{CommonName, UniqueName, Data};
use crate::Relic;
use crate::recipe;

#[derive(Debug)]
pub struct Requirement
{
	common_name: CommonName,
	unique_name: UniqueName,
	requirement_type: RequirementType,
}

impl Requirement
{
	pub fn new(unique_name: UniqueName, db: &Data) -> Result<Self>
	{
		let common_name = db.resource_common_name(unique_name.clone())
			.context("Looking for common name")?;
		let requirement_type = match db.recipe(unique_name.clone())
		{
			// Craft component
			Some(recipe_unique_name) =>
			{
				let recipe = recipe::Recipe::new(db, recipe_unique_name)?;
				RequirementType::CraftComponent(recipe)
			},

			None=>
			{
				// let available_from_invasion = db.available_from_invasion(unique_name);
				let component = Component::new(unique_name.clone(), common_name.clone(), db);
				RequirementType::Component(component)
			}
		};

		Ok(Self{common_name, unique_name, requirement_type})
	}

	pub fn common_name(&self) -> CommonName
	{
		self.common_name.clone()
	}

	pub fn unique_name(&self) -> UniqueName
	{
		self.unique_name.clone()
	}

	pub fn active_relics(&self) -> &[Relic]
	{
		match &self.requirement_type
		{
			RequirementType::Component(Component::Prime(ref pc))=>
			{
				&pc.active_relics
			},

			RequirementType::CraftComponent(cc)=>
			{
				cc.active_relics()
			},

			_ => &[]
		}
	}

	pub fn resurgence_relics(&self) -> &[Relic]
	{
		match &self.requirement_type
		{
			RequirementType::Component(Component::Prime(ref pc))=>
			{
				&pc.resurgence_relics
			},

			RequirementType::CraftComponent(cc)=>
			{
				cc.resurgence_relics()
			},

			_ => &[]
		}
	}

	pub fn available_from_invasion(&self) -> bool
	{
		match &self.requirement_type
		{
			RequirementType::Component(Component::Normal(ref nc))=>
			{
				nc.available_from_invasion
			},

			RequirementType::CraftComponent(cc)=>
			{
				cc.available_from_invasion()
			},

			_ => false
		}
	}
}

#[derive(Debug)]
enum RequirementType
{
	Component(Component),
	CraftComponent(recipe::Recipe),
}

#[derive(Debug)]
enum Component
{
	Normal(NormalComponent),
	Prime(PrimeComponent)
}

impl Component
{
	pub fn new(unique_name: UniqueName, common_name: CommonName, db: &Data) -> Self
	{
		if common_name.as_str().contains("Prime")
		{
			let active_relics = db.active_relics(unique_name.as_str())
				.unwrap_or_default();
			let resurgence_relics = db.resurgence_relics(unique_name.as_str())
				.unwrap_or_default();
			let prime_component = PrimeComponent
			{
				active_relics,
				resurgence_relics
			};
			Self::Prime(prime_component)
		}
		else
		{
			let available_from_invasion = db.available_from_invasion(unique_name);
			let normal_component = NormalComponent
			{
				available_from_invasion
			};
			Self::Normal(normal_component)
		}
	}
}

#[derive(Debug)]
struct NormalComponent
{
	available_from_invasion: bool
}

#[derive(Debug)]
struct PrimeComponent
{
	active_relics: Vec<Relic>,
	resurgence_relics: Vec<Relic>,
}