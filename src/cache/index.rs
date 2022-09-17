use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};

pub(crate) fn load(path: &Path) -> Result<HashMap<String, String>>
{
	std::fs::read_to_string(path)
		.context("Loading manifest file")?
		.lines()
		.map(|l|(&l[0..l.len()-26], l))
		.map(|(k, v)|Ok((k.to_string(), v.to_string())))
		.collect()
}