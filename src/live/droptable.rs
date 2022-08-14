use anyhow::Result;

const DROPTABLE: &str = "https://www.warframe.com/droptables";

pub(crate) fn droptable() -> Result<String>
{
	ureq::get(DROPTABLE)
		.call()?
		.into_string()
		.map_err(|e|e.into())
}

