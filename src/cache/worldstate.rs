use std::{path::Path, io::BufReader, fs::File};
use scraper::{Html, Selector};
use anyhow::Result;
use serde::Deserialize;

pub(crate) fn active_relics(file_path: &Path) -> Result<Vec<String>>
{
	if !file_path.exists()
	{
		let table = crate::live::droptable()?;
		std::fs::write(file_path, &table)?;
	}
	let contents = std::fs::read_to_string(&file_path)?;
	let parsed = Html::parse_document(&contents);
	let table_selector = Selector::parse(r#"#missionRewards~table"#).unwrap();
	let table = parsed.select(&table_selector).next().unwrap();
	let relic_selector = Selector::parse(r#"td"#).unwrap();
	let relics = table.select(&relic_selector)
		.flat_map(|e|e.text().next())
		.filter(|r|
		{
			r.starts_with("Lith")||
			r.starts_with("Meso")||
			r.starts_with("Neo")||
			r.starts_with("Axi")
		})
		.map(|r|r.trim_end_matches(" (Radiant)"))
		.map(|r|r.to_string())
		.collect();
	Ok(relics)
}

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

pub(crate) fn resurgence_relics(file_path: &Path) -> Result<Vec<String>>
{
	if !file_path.exists()
	{
		let worldstate = crate::live::worldstate()?;
		std::fs::write(file_path, &worldstate)?;
	}

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