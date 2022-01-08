use std::io;
use std::io::prelude::*;



use lzma_rs::lzma_decompress;
const EXPORT: &str = "https://content.warframe.com/PublicExport";
const MANIFEST_TEMPLATE: &str = "https://content.warframe.com/PublicExport/Manifest";
const DROP_TABLE_URL: &str = "https://www.warframe.com/droptables";


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

pub fn drop_table_relics() -> io::Result<()>
{
	// use html_parser::Dom;

	// let response = ureq::get(DROP_TABLE_URL)
	// 	.call()
	// 	.map_err(|e|io::Error::new(io::ErrorKind::Other, e))?
	// 	.into_string()?;
	let response = std::fs::read_to_string(r"C:\Users\brian\primes\drops.html")?;
	let start = r#"<h3 id="relicRewards">Relics:</h3>"#;
	let starti = response.find(start).unwrap() + start.len();
	let interest = &response[starti..];
	let end = "</table>";
	let endi = interest.find(end).unwrap() + end.len();
	let interest = &interest[..endi];
	let interest = interest.trim_start();
	println!("{:?}", interest);

	// let mut reader = Reader::from_str(&response);
	// let mut buf = vec![];
	// loop {
	// 	buf.clear();
	// 	reader.read_event(&mut buf).unwrap();
	// 	println!("{}", String::from_utf8_lossy(&buf));
	// }
	Ok(())
}