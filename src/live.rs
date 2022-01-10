use std::io;



use lzma_rs::lzma_decompress;
const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";


// fn main() -> std::io::Result<()>{

// 	for endpoint in load_endpoints()?
// 	{
// 		let manifest_url = format!("{}/{}", ENDPOINT_TEMPLATE, endpoint);
// 		let manifest = load_manifest(&manifest_url)?;
// 		std::fs::write(format!("/home/brian/primes/src/{}", endpoint), &manifest)?;
// 	}
// 	Ok(())
// }

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

pub fn index() -> io::Result<String>
{
	let index_url = format!("{}/{}", EXPORT, "index_en.txt.lzma");
	let response = ureq::get(&index_url)
		.call()
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
	let mut decompressed = Vec::new();
	lzma_decompress(
		&mut io::BufReader::new(response.into_reader()),
		&mut decompressed).unwrap();
	String::from_utf8(decompressed)
		.map_err(|e|io::Error::new(io::ErrorKind::Other, e))
}