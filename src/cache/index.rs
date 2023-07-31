use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use anyhow::{Context, Result};

pub fn load(path: &Path) -> Result<HashMap<String, String>>
{
	let file = File::open(path)
		.context("Opening compressed manifest file")?;
	let mut reader = BufReader::new(file);
	parse(&mut reader)
}

pub fn parse<R>(reader: &mut R) -> Result<HashMap<String, String>>
where
	R: BufRead
{
	let mut decompressed = vec![];
	lzma_rs::lzma_decompress(reader, &mut decompressed)
		.context("Decompressing manifest file")?;
	let decoded = String::from_utf8(decompressed)
		.context("Parsing decompressed manifest file")?;
	decoded.lines()
		.map(|l|(&l[0..l.len()-26], l))
		.map(|(k, v)|Ok((k.to_string(), v.to_string())))
		.collect()
}