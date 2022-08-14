use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

pub(crate) fn load(path: &Path) -> Result<HashMap<String, String>>
{
	let i = std::fs::read_to_string(path)?;
	let index = i.lines()
		.map(|l|(&l[0..l.len()-26], l))
		.map(|(k, v)|(k.to_string(), v.to_string()))
		.collect();
	Ok(index)
}