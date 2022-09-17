use std::io;
use std::path::Path;
use std::collections::HashMap;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Eq, PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
struct Saved
{
	tracked: Vec<String>,
	owned: HashMap<String, u32>,
}

pub(crate) fn load(
	tracked_path: &Path,
	db: &mut db::Database) -> Result<(Vec<crate::Tracked>, HashMap<String, u32>)>
{
	let contents = std::fs::read_to_string(tracked_path)
		.context("Loading tracked file from fs")?;
	let parsed: Saved = serde_json::from_str(&contents)
		.context("Parsing tracked")?;
	let t = parsed.tracked.into_iter()
		.map(|t|crate::Tracked::new(db, t))
		.collect::<Result<Vec<crate::Tracked>>>()?;
	Ok((t, parsed.owned))
}

pub(crate) fn save(
	tracked_path: &Path,
	tracked: &[crate::Tracked],
	owned: &HashMap<String, u32>) -> Result<()>
{
	let t: Vec<_> = tracked
		.iter()
		.map(|t|t.unique_name.clone())
		.collect();
	let saved = Saved {tracked: t, owned: owned.clone()};
	let file = std::fs::File::create(tracked_path)
		.context("Creating tracked file")?;
	let mut buf = io::BufWriter::new(file);
	serde_json::to_writer(&mut buf, &saved)
		.context("Writing to tracked file")
}