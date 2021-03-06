use std::io;
use std::collections::HashMap;

use anyhow::Result;
use lzma_rs::lzma_decompress;

const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";

pub(crate) fn load_manifest(name: &str) -> Result<String>
{
	let url = format!("{}/{}", MANIFEST_TEMPLATE, name);
	ureq::get(&url)
		.call()?
		.into_string()
		.map_err(|e|e.into())
}

pub fn index() -> Result<HashMap<String, String>>
{
	let index_url = format!("{}/{}", EXPORT, "index_en.txt.lzma");
	let response = ureq::get(&index_url)
		.call()
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
	let mut decompressed = Vec::new();
	lzma_decompress(
		&mut io::BufReader::new(response.into_reader()),
		&mut decompressed).unwrap();
	let i = String::from_utf8(decompressed)
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
	let index = i.lines()
		.map(|l|(&l[0..l.len()-26], &l[..]))
		.map(|(k, v)|(k.to_string(), v.to_string()))
		.collect();
	Ok(index)
}