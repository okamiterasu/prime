use anyhow::Result;

const WORLDSTATE: &str = "https://content.warframe.com/dynamic/worldState.php";

pub fn worldstate() -> Result<String>
{
	let response = ureq::get(WORLDSTATE)
		.call()?;
	response.into_string().map_err(|e|e.into())
}