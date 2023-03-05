use anyhow::{Result, Context};

const DROPTABLE: &str = "https://www.warframe.com/droptables";

pub fn droptable() -> Result<String>
{
	ureq::get(DROPTABLE)
		.call()
		.context("Sending GET request for the droptable")?
		.into_string()
		.context("Parsing response from droptable GET")
}

