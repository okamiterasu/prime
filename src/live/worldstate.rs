use anyhow::{Result, Context};

const WORLDSTATE: &str = "https://content.warframe.com/dynamic/worldState.php";

pub fn worldstate() -> Result<String>
{
	ureq::get(WORLDSTATE)
		.call()
		.context("Sending GET request for the worldstate")?
		.into_string()
		.context("Parsing response from worldstate GET")
}