use std::rc::Rc;
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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


// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct ResultType(pub Rc<str>);

// impl From<String> for ResultType
// {
// 	fn from(i: String) -> Self
// 	{
// 		Self(i.into())
// 	}
// }

// impl From<&str> for ResultType
// {
// 	fn from(i: &str) -> Self
// 	{
// 		Self(i.into())
// 	}
// }


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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