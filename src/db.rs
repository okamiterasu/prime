use std::collections::HashSet;
use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use crate::{relics, live, setup};

enum Ternary
{
	True,
	False,
	Whatever
}

pub fn open(path: &Path, force_refresh: bool) -> rusqlite::Result<Connection>
{
	if !path.exists() || force_refresh {
		init(path)
	} else {
		Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
	}
}

fn init(path: &Path) -> rusqlite::Result<Connection>
{
	// Try to delete file, but don't make a fuss if it fails
	std::fs::remove_file(path).unwrap_or(());
	let mut db = Connection::open(path)?;
	let t = db.transaction()?;

	// Create Tables
	t.execute(r#"CREATE TABLE IF NOT EXISTS WARFRAME
	(
		name		TEXT PRIMARY KEY,
		unique_name	TEXT NOT NULL
	)"#, [])?;
	t.execute(r#"CREATE TABLE IF NOT EXISTS WEAPON
	(
		name		TEXT PRIMARY KEY,
		unique_name	TEXT NOT NULL
	)"#, [])?;
	t.execute(r#"CREATE TABLE IF NOT EXISTS RECIPE
	(
		unique_name	TEXT PRIMARY KEY,
		result_type TEXT NOT NULL
	)"#, [])?;
	t.execute(r#"CREATE TABLE IF NOT EXISTS REQUIRES
	(
		recipe_unique_name TEXT NOT NULL,
		item_type	TEXT NOT NULL,
		item_count	INT NOT NULL,
		CONSTRAINT PK PRIMARY KEY (recipe_unique_name, item_type)
	)"#, [])?;
	t.execute(r#"CREATE TABLE IF NOT EXISTS RELIC
	(
		unique_name TEXT PRIMARY KEY,
		name		TEXT NOT NULL,
		active		BOOLEAN DEFAULT FALSE,
		resurgence	BOOLEAN DEFAULT FALSE
	)"#, [])?;
	t.execute(r#"CREATE TABLE IF NOT EXISTS RELIC_REWARD
	(
		relic 	TEXT NOT NULL,
		name	TEXT NOT NULL,
		rarity	INT NOT NULL,
		CONSTRAINT PK PRIMARY KEY (relic, name)
	)"#, [])?;
	t.execute(r#"CREATE TABLE IF NOT EXISTS RESOURCE
	(
		unique_name TEXT PRIMARY KEY,
		name		TEXT NOT NULL
	)"#, [])?;
	// Populate Tables
	for manifest in live::index().unwrap()
	{
		let m = live::load_manifest(&manifest).unwrap();
		let ms = manifest.split("!00_").next().unwrap();
		std::fs::write(path.parent().unwrap().join(ms), &m).unwrap();
	}
	setup::weapons(&t, &path.parent().unwrap().join("ExportWeapons_en.json"))?;
	setup::warframes(&t, &path.parent().unwrap().join("ExportWarframes_en.json"))?;
	setup::recipes(&t, &path.parent().unwrap().join("ExportRecipes_en.json"))?;
	setup::resources(&t, &path.parent().unwrap().join("ExportResources_en.json"))?;
	setup::relics(&t, &path.parent().unwrap().join("ExportRelicArcane_en.json"))?;
	t.commit()?;


	Ok(db)
}

pub fn requirements(db: &Connection, recipe_unique_name: &str) -> rusqlite::Result<Vec<(Option<String>, String, u32)>>
{
	let mut components = db.prepare(r#"
	SELECT RESOURCE.name, REQUIRES.item_type, REQUIRES.item_count
	FROM REQUIRES
		INNER JOIN RESOURCE
			ON REQUIRES.item_type = RESOURCE.unique_name
	WHERE REQUIRES.recipe_unique_name = ?
	ORDER BY
		item_count ASC,
		REQUIRES.item_type ASC"#)?;
		
	let response = components.query([recipe_unique_name])?
		.mapped(|r|{
			let common_name = r.get(0)?;
			let unique_name = r.get(1)?;
			let count = r.get(2)?;
			Ok((common_name, unique_name, count))})
		.flatten()
		.collect();
	Ok(response)
}

pub fn common_name(db: &Connection, unique_name: &str) -> rusqlite::Result<Option<String>>
{
	let mut query = db.prepare(r#"
		SELECT name
		FROM (
			SELECT name, unique_name
				FROM WARFRAME
			UNION
			SELECT name, unique_name
				FROM WEAPON
			UNION
			SELECT name, unique_name
				FROM RESOURCE
		) T1
		WHERE unique_name = ?"#)?;
	let response = query
		.query([unique_name])?
		.mapped(|r|r.get(0))
		.flatten()
		.next();
	Ok(response)
}

pub fn unique_name(db: &Connection, common_name: &str) -> rusqlite::Result<String>
{
	let mut query = db.prepare(r#"
		SELECT unique_name
			FROM (
				SELECT name, unique_name
					FROM WARFRAME
				UNION
				SELECT name, unique_name
					FROM WEAPON
				UNION
				SELECT name, unique_name
					FROM RESOURCE
			) T1
		WHERE name = ?"#)?;
	let response: String = query.query_row([common_name], |r|r.get(0))?;
	Ok(response)
}

pub fn how_many_needed(db: &Connection, recipe_unique_name: &str, resource_unique_name: &str) -> rusqlite::Result<u32>
{
	let mut query = db.prepare(r#"
		SELECT REQUIRES.item_count
			FROM REQUIRES
		WHERE
			REQUIRES.recipe_unique_name = ?1 AND
			REQUIRES.item_type = ?2"#)?;
	let response: u32 = query.query_row(
		[recipe_unique_name, resource_unique_name],
		|r|r.get(0))?;
	Ok(response)
}

pub fn active_relics(db: &Connection, unique_name: &str) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	let mut relics = HashSet::new();
	let component_relics = component_relics(db, unique_name, Ternary::True, Ternary::Whatever)?;
	relics.extend(component_relics);
	let recipe_relics = recipe_relics(db, unique_name, Ternary::True, Ternary::Whatever)?;
	relics.extend(recipe_relics);
	Ok(relics.into_iter().collect())
}

pub fn resurgence_relics(db: &Connection, unique_name: &str) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	let mut relics = HashSet::new();
	let component_relics = component_relics(db, unique_name, Ternary::Whatever, Ternary::True)?;
	relics.extend(component_relics);
	let recipe_relics = recipe_relics(db, unique_name, Ternary::Whatever, Ternary::True)?;
	relics.extend(recipe_relics);
	Ok(relics.into_iter().collect())
}

fn component_relics(db: &Connection, component_unique_name: &str, active: Ternary, resurgence: Ternary) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	let mut statement = r#"
	SELECT RELIC.name, RELIC_REWARD.rarity
		FROM RELIC_REWARD
			INNER JOIN RELIC
				ON RELIC_REWARD.relic = RELIC.unique_name
	WHERE
		RELIC_REWARD.name = ?1"#.to_string();

	match active
	{
		Ternary::True=>statement.push_str(" AND RELIC.active = TRUE"),
		Ternary::False=>statement.push_str(" AND RELIC.active = FALSE"),
		Ternary::Whatever=>(),
	}

	match resurgence
	{
		Ternary::True=>statement.push_str(" AND RELIC.resurgence = TRUE"),
		Ternary::False=>statement.push_str(" AND RELIC.resurgence = FALSE"),
		Ternary::Whatever=>(),
	}

	let mut query = db.prepare(&statement)?;

	let response = query.query([component_unique_name])?
		.mapped(|r|{
			let relic = r.get(0)?;
			let rarity: String = r.get(1)?;
			let rarity = relics::Rarity::try_from(&*rarity).unwrap();
			Ok((relic, rarity))})
		.flatten()
		.collect();
	Ok(response)
}

fn recipe_relics(db: &Connection, result_unique_name: &str, active: Ternary, resurgence: Ternary) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	let mut statement = r#"
	SELECT RELIC.name, RELIC_REWARD.rarity
		FROM RECIPE
			INNER JOIN RELIC_REWARD
				ON RECIPE.unique_name = RELIC_REWARD.name
			INNER JOIN RELIC
				ON RELIC_REWARD.relic = RELIC.unique_name
	WHERE RECIPE.result_type = ?"#.to_string();

	match active
	{
		Ternary::True=>statement.push_str(" AND RELIC.active = TRUE"),
		Ternary::False=>statement.push_str(" AND RELIC.active = FALSE"),
		Ternary::Whatever=>(),
	}

	match resurgence
	{
		Ternary::True=>statement.push_str(" AND RELIC.resurgence = TRUE"),
		Ternary::False=>statement.push_str(" AND RELIC.resurgence = FALSE"),
		Ternary::Whatever=>(),
	}

	let mut query = db.prepare(&statement)?;

	let response = query.query([result_unique_name])?
		.mapped(|r|{
			let relic = r.get(0)?;
			let rarity: String = r.get(1)?;
			let rarity = relics::Rarity::try_from(&*rarity).unwrap();
			Ok((relic, rarity))})
		.flatten()
		.collect();
	Ok(response)
}

pub fn recipe(db: &Connection, result_unique_name: &str) -> rusqlite::Result<String>
{
	let mut query = db.prepare(r#"
		SELECT unique_name
			FROM RECIPE
		WHERE result_type = ?"#)?;	
	query.query_row([result_unique_name],|r|r.get(0))
}