use std::io;
use std::collections::HashSet;
use std::path::Path;

use scraper::{Html, Selector};

use crate::live;

pub fn active_relics(file_path: &Path) -> io::Result<HashSet<String>>
{
	if !file_path.exists()
	{
		let table = live::droptable()?;
		std::fs::write(file_path, &table).unwrap();
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
		.map(|r|r.to_ascii_uppercase())
		.collect();
	Ok(relics)
}

