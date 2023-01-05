use anyhow::{anyhow, Result, Error};
use serde::Deserialize;

use crate::structures::CommonName;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub enum Rarity
{
	Common,
	Uncommon,
	Rare
}
impl TryFrom<&str> for Rarity
{
	type Error = Error;
	fn try_from(i: &str) -> Result<Self, Self::Error>
	{
		match i
		{
			"COMMON"=>Ok(Self::Common),
			"UNCOMMON"=>Ok(Self::Uncommon),
			"RARE"=>Ok(Self::Rare),
			_=>Err(anyhow!("Unknown rarity: {}", i))
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relic
{
	name: CommonName,
	pub rarity: Rarity
}

impl Relic
{
	pub fn new(name: CommonName, rarity: Rarity) -> Self
	{
		Self{name, rarity}
	}

	pub fn name(&self) -> &str
	{
		self.name.as_str()
	}
}