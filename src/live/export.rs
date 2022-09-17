use std::io;

use anyhow::{Result, Context};

const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";

pub(crate) fn load_manifest(name: &str) -> Result<String>
{
	let url = format!("{}/{}", MANIFEST_TEMPLATE, name);
	ureq::get(&url)
		.call()
		.with_context(||format!("Sending GET request for manifest {name}"))?
		.into_string()
		.with_context(||format!("Parsing manifest {name} as a String"))
}

pub fn index() -> Result<String>
{
	let index_url = format!("{EXPORT}/index_en.txt.lzma");
	let response = ureq::get(&index_url)
		.call()
		.context("Sending GET request for manifest index")?;
	let mut decompressed = Vec::new();
	lzma_rs::lzma_decompress(
		&mut io::BufReader::new(response.into_reader()),
		&mut decompressed)
		.context("Decompressing manifest index")?;
	String::from_utf8(decompressed)
		.context("Parsing decompressed manifest index")
}