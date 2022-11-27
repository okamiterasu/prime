use std::path::Path;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct State
{
	prime_vault_traders: Vec<PrimeVaultTrader>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PrimeVaultTrader
{
	pub manifest: Vec<Item>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item
{
	pub item_type: String,
}

pub fn resurgence_relics(file_path: &Path) -> Result<Vec<String>>
{
	use std::fs::File;
	use std::io::BufReader;

	if !file_path.exists()
	{
		let worldstate = crate::live::worldstate()?;
		std::fs::write(file_path, &worldstate)?;
	}

	let reader = BufReader::new(File::open(file_path)?);
	let world_state: State = serde_json::from_reader(reader)?;
	let store_items = world_state.prime_vault_traders.iter()
		.flat_map(|t|&t.manifest)
		.map(|i|&i.item_type[..])
		.filter(|i|i.starts_with("/Lotus/StoreItems/Types/Game/Projections/"));
	// Resurgence store items contain a "StoreItems" node in the path which
	// needs to be removed to make it line up with the rest of the names.
	// TODO: I feel like there should be a better way to do this.
	let normalized_store_items = store_items
		.map(|i|i.split('/').filter(|s|*s != "StoreItems").collect());
	let relics = normalized_store_items
		.map(|i: Vec<_>|i.join("/"))
		.collect();
	Ok(relics)
}