use anyhow::{Result, Context};

const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";

pub(crate) fn manifest(name: &str) -> Result<String>
{
	let url = format!("{MANIFEST_TEMPLATE}/{name}");
	ureq::get(&url)
		.call()
		.context("Sending GET request")?
		.into_string()
		.context("Parsing manifest as a String")
}

pub fn index() -> Result<Vec<u8>>
{
	let index_url = format!("{EXPORT}/index_en.txt.lzma");
	let response = ureq::get(&index_url)
		.call()
		.context("Sending GET request for manifest index")?;
	let mut payload = vec![];
	response.into_reader()
		.read_to_end(&mut payload)
		.context("Reading response payload")?;
	Ok(payload)
}