// use std::io::prelude::*;
use std::io;
use io::BufReader;
use std::fs::File;
use std::path::{Path};
use std::collections::HashSet;

use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
struct State
{
	prime_vault_traders: Vec<PrimeVaultTrader>
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PrimeVaultTrader
{
	pub manifest: Vec<Item>
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Item
{
	pub item_type: String,
}

pub fn resurgence_relics(file_path: &Path) -> std::io::Result<HashSet<String>>
{
	let reader = BufReader::new(File::open(file_path)?);
	let world_state: State = serde_json::from_reader(reader)?;
	let relics = world_state.prime_vault_traders.iter()
		.flat_map(|t|&t.manifest)
		.map(|i|&i.item_type[..])
		.filter(|i|i.starts_with("/Lotus/StoreItems/Types/Game/Projections/"))
		.map(|i|i.split('/'))
		.map(|i|i.filter(|s|*s != "StoreItems"))
		.map(|i|i.collect())
		.map(|i: Vec<_>|i.join("/"))
		.collect();
	Ok(relics)
}