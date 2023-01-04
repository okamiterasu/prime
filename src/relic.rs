use anyhow::{anyhow, Result, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	name: String,
	pub rarity: Rarity
}

impl Relic
{
	pub fn new(name: &str, rarity: &str) -> Result<Self>
	{
		let rarity = Rarity::try_from(rarity)?;
		let x = Self
		{
			name: name.to_string(),
			rarity
		};
		Ok(x)
	}

	pub fn name(&self) -> &str
	{
		&self.name
	}
}