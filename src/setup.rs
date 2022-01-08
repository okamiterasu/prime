use std::path::Path;
use rusqlite::{params, Connection};


pub fn db(path: &Path) -> rusqlite::Result<Connection>
{

	let db = Connection::open(path)?;
	db.execute(r#"CREATE TABLE IF NOT EXISTS WARFRAME
	(
		name		TEXT PRIMARY KEY,
		unique_name	TEXT NOT NULL
	)"#, [])?;

	db.execute(r#"CREATE TABLE IF NOT EXISTS WEAPON
	(
		name		TEXT PRIMARY KEY,
		unique_name	TEXT NOT NULL
	)"#, [])?;

	db.execute(r#"CREATE TABLE IF NOT EXISTS RECIPE
	(
		unique_name	TEXT PRIMARY KEY,
		result_type TEXT NOT NULL
	)"#, [])?;

	db.execute(r#"CREATE TABLE IF NOT EXISTS REQUIRES
	(
		result_type TEXT NOT NULL,
		item_type	TEXT NOT NULL,
		item_count	INT NOT NULL,
		name		TEXT,
		CONSTRAINT PK PRIMARY KEY (result_type, item_type)
	)"#, [])?;

	db.execute(r#"CREATE TABLE IF NOT EXISTS RELIC
	(
		unique_name TEXT PRIMARY KEY,
		name		TEXT NOT NULL
	)"#, [])?;

	db.execute(r#"CREATE TABLE IF NOT EXISTS RELIC_REWARD
	(
		relic 	TEXT NOT NULL,
		name	TEXT NOT NULL,
		rarity	INT NOT NULL,
		CONSTRAINT PK PRIMARY KEY (relic, name)
	)"#, [])?;

	Ok(db)
}

pub fn recipes(cache_dir: &Path, endpoints: &[String], db: &mut Connection) -> rusqlite::Result<()>
{
	let recipe_endpoint = endpoints.iter()
		.find(|e|e.contains("Recipes"))
		.expect("No Recipe endpoint found");
	let recipe_path = cache_dir.join(recipe_endpoint);
	if !recipe_path.exists()
	{
		let manifest = super::live::load_manifest(&recipe_endpoint).unwrap();
		std::fs::write(&recipe_path, &manifest).unwrap();
	}

	let t = rusqlite::Transaction::new(db, rusqlite::TransactionBehavior::Deferred)?;
	for recipe in super::recipes::parse_from_file(&cache_dir.join(recipe_endpoint)).unwrap()
	{
		t.execute(r#"
			INSERT OR REPLACE INTO RECIPE (unique_name, result_type)
			VALUES (?1, ?2)"#,
			params![recipe.unique_name, recipe.result_type])?;
		for ingredient in recipe.ingredients
		{
			let name = normalize_component_name(&ingredient.item_type);

			t.execute(r#"
				INSERT OR REPLACE INTO REQUIRES (result_type, item_type, name, item_count)
				VALUES (?1, ?2, ?3, ?4)"#,
				params![recipe.result_type, ingredient.item_type, name, ingredient.item_count])?;
		}
	}
	t.commit()?;
	Ok(())
}

pub fn warframes(cache_dir: &Path, endpoints: &[String], db: &mut Connection) -> rusqlite::Result<()>
{
	let warframe_endpoint = endpoints.iter()
		.find(|e|e.contains("Warframes"))
		.expect("No Warframe endpoint found");
	let warframe_path = cache_dir.join(warframe_endpoint);
	if !warframe_path.exists()
	{
		let manifest = super::live::load_manifest(&warframe_endpoint).unwrap();
		std::fs::write(&warframe_path, &manifest).unwrap();
	}
	let t = rusqlite::Transaction::new(db, rusqlite::TransactionBehavior::Deferred)?;
	for warframe in super::frames::parse_from_file(&cache_dir.join(warframe_endpoint)).unwrap()
	{
		if warframe.name.starts_with("<ARCHWING>") || !warframe.name.contains("PRIME")
		{
			continue
		}
		t.execute(r#"
		INSERT OR REPLACE INTO WARFRAME (unique_name, name)
		VALUES (?1, ?2)"#,
		params![warframe.unique_name, warframe.name])?;
	}
	t.commit()?;
	Ok(())
}

pub fn weapons(cache_dir: &Path, endpoints: &[String], db: &mut Connection) -> rusqlite::Result<()>
{
	let weapon_endpoint = endpoints.iter()
		.find(|e|e.contains("Weapon"))
		.expect("No Weapon endpoint found");

	let weapon_path = cache_dir.join(weapon_endpoint);
	if !weapon_path.exists()
	{
		let manifest = super::live::load_manifest(&weapon_endpoint).unwrap();
		std::fs::write(&weapon_path, &manifest).unwrap();
	}

	let t = rusqlite::Transaction::new(db, rusqlite::TransactionBehavior::Deferred)?;
	for weapon in super::weapons::parse_from_file(&cache_dir.join(weapon_endpoint)).unwrap()
	{
		if !weapon.name.contains("PRIME")
		{
			continue
		}
		t.execute(r#"
		INSERT OR REPLACE INTO WEAPON (unique_name, name)
		VALUES (?1, ?2)"#,
		params![weapon.unique_name, weapon.name])?;
	}
	t.commit()?;
	Ok(())
}

pub fn relics(cache_dir: &Path, endpoints: &[String], db: &mut Connection) -> rusqlite::Result<()>
{
	let relic_endpoint = endpoints.iter()
		.find(|e|e.contains("Relic"))
		.expect("No relic endpoint found");

	let relic_path = cache_dir.join(relic_endpoint);
	if !relic_path.exists()
	{
		let manifest = super::live::load_manifest(&relic_endpoint).unwrap();
		std::fs::write(&relic_path, &manifest).unwrap();
	}

	let t = rusqlite::Transaction::new(db, rusqlite::TransactionBehavior::Deferred)?;
	for relic in super::relics::parse_from_file(&cache_dir.join(relic_endpoint)).unwrap()
	{
		t.execute(r#"
		INSERT OR REPLACE INTO RELIC (unique_name, name)
		VALUES (?1, ?2)"#,
		params![relic.unique_name, relic.name])?;
		for reward in relic.relic_rewards
		{
			t.execute(r#"
				INSERT OR REPLACE INTO RELIC_REWARD (relic, name, rarity)
				VALUES (?1, ?2, ?3)"#,
				params![relic.name, reward.reward_name, reward.rarity.as_str()])?;
		}
	}
	t.commit()?;
	Ok(())
}

fn normalize_component_name(input: &str) -> &str
{
	let mut name = input.rsplit('/').next().unwrap();
	if !name.contains("Prime")
	{
		name = name.strip_suffix("Item").unwrap_or(name);
		return name
	}
	name = input.strip_suffix("Component").unwrap_or(name);
	let i = name.rfind(|c: char|c.is_uppercase()).unwrap();
	&name[i..]
}

