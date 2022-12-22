use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};

pub(crate) fn load(path: &Path) -> Result<HashMap<String, String>>
{
	let file = File::open(path)
		.context("Opening compressed manifest file")?;
	let mut reader = BufReader::new(file);
	let mut decompressed = vec![];
	lzma_rs::lzma_decompress(&mut reader, &mut decompressed)
		.context("Decompressing manifest file")?;
	let parsed = String::from_utf8(decompressed)
		.context("Parsing decompressed manifest file")?;
	parsed
		.lines()
		.map(|l|(&l[0..l.len()-26], l))
		.map(|(k, v)|Ok((k.to_string(), v.to_string())))
		.collect()
}