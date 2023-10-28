use anyhow::{Result, Context};

const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";

pub fn manifest(name: &str) -> Result<String>
{
	println!("Downloading new manifest: {name}");
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
	let payload_size: usize = response.header("Content-Length")
		.context("Could not read Content-Length")
		.and_then(|cl|cl.parse().context("Could not parse Content-Length"))
		.unwrap_or(0);
	let mut payload = Vec::with_capacity(payload_size);
	response.into_reader()
		.read_to_end(&mut payload)
		.context("Reading response payload")?;
	Ok(payload)
}