use std::io;



use lzma_rs::lzma_decompress;
const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";
const WORLDSTATE: &str = "https://content.warframe.com/dynamic/worldState.php";
const DROPTABLE: &str = "https://www.warframe.com/droptables";

pub fn load_manifest(name: &str) -> io::Result<String>
{
	let url = format!("{}/{}", MANIFEST_TEMPLATE, name);
	let response = ureq::get(&url)
		.call()
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
	response.into_string()
		.map(|r|r.replace(r"\r", ""))
		.map(|r|r.replace(&['\r', '\n'][..], ""))
}

pub fn index() -> io::Result<Vec<String>>
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

	Ok(i.lines().map(|l|l.to_string()).collect())
}

pub fn worldstate() -> io::Result<String>
{
	let response = ureq::get(WORLDSTATE)
		.call()
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
	response.into_string()
}

pub fn droptable() -> io::Result<String>
{
	let response = ureq::get(DROPTABLE)
		.call()
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
	response.into_string()
}