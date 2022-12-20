use std::path::Path;

use anyhow::{anyhow, Result};

fn is_relic(item: &str) -> bool
{
	item.starts_with("Lith")
	||item.starts_with("Meso")
	||item.starts_with("Neo")
	||item.starts_with("Axi")
}

pub fn active_relics(file_path: &Path) -> Result<Vec<String>>
{
	use scraper::{Html, Selector};

	if !file_path.exists()
	{
		let table = crate::live::droptable()?;
		std::fs::write(file_path, table)?;
	}
	let contents = std::fs::read_to_string(file_path)?;
	let parsed = Html::parse_document(&contents);
	let table_selector = Selector::parse(r#"#missionRewards~table"#).unwrap();
	let table = parsed.select(&table_selector).next()
		.ok_or_else(||anyhow!("Could not find the mission rewards table"))?;
	let relic_selector = Selector::parse(r#"td"#).unwrap();
	let relics = table.select(&relic_selector)
		.flat_map(|e|e.text().next())
		.filter(|r|is_relic(r))
		.map(|r|r.trim_end_matches(" (Radiant)"))
		.map(|r|r.to_string())
		.collect();
	Ok(relics)
}