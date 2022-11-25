use std::rc::Rc;
use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(into = "String")]
#[serde(from = "String")]
pub struct UniqueName(pub Rc<str>);

impl UniqueName
{
	pub fn as_str(&self) -> &str
	{
		&self.0
	}
}

impl Display for UniqueName
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<String> for UniqueName
{
	fn from(i: String) -> Self
	{
		Self(i.into())
	}
}

impl From<&str> for UniqueName
{
	fn from(i: &str) -> Self
	{
		Self(i.into())
	}
}

impl From<UniqueName> for String
{
	fn from(i: UniqueName) -> Self
	{
		i.0.to_string()
	}
}

impl From<CommonName> for UniqueName
{
	fn from(i: CommonName) -> Self
	{
		Self(i.0)
	}
}

impl PartialEq<str> for UniqueName
{
	fn eq(&self, other: &str) -> bool
	{
		self.as_str() == other
	}
}

impl PartialEq<UniqueName> for str
{
	fn eq(&self, other: &UniqueName) -> bool
	{
		self == other.as_str()
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommonName(Rc<str>);

impl CommonName
{
	pub fn as_str(&self) -> &str
	{
		&self.0
	}
}

impl Display for CommonName
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<String> for CommonName
{
	fn from(i: String) -> Self
	{
		Self(i.into())
	}
}

impl From<&str> for CommonName
{
	fn from(i: &str) -> Self
	{
		Self(i.into())
	}
}

impl From<UniqueName> for CommonName
{
	fn from(i: UniqueName) -> Self
	{
		Self(i.0)
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Count(pub u32);

impl Count
{
	pub fn to_u32(&self) -> u32
	{
		self.0
	}
}

impl From<u32> for Count
{
	fn from(i: u32) -> Self
	{
		Self(i)
	}
}

impl Display for Count
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.0)
	}
}