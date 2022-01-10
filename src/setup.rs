use std::path::{Path};
use druid::LensExt;
use rusqlite::{params, Connection, Transaction};
use crate::db;


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
		recipe_unique_name TEXT NOT NULL,
		item_type	TEXT NOT NULL,
		item_count	INT NOT NULL,
		CONSTRAINT PK PRIMARY KEY (recipe_unique_name, item_type)
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

	db.execute(r#"CREATE TABLE IF NOT EXISTS RESOURCE
	(
		unique_name TEXT PRIMARY KEY,
		name		TEXT NOT NULL
	)"#, [])?;

	db.execute(r#"CREATE TABLE IF NOT EXISTS BLUEPRINT
	(
		unique_name TEXT PRIMARY KEY,
		name		TEXT NOT NULL
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
	let recipes = super::recipes::parse_from_file(&cache_dir.join(recipe_endpoint)).unwrap();
	for recipe in &recipes
	{
		t.execute(r#"
			INSERT OR REPLACE INTO RECIPE (unique_name, result_type)
			VALUES (?1, ?2)"#,
			params![
				recipe.unique_name,
				recipe.result_type])?;		

		for ingredient in &recipe.ingredients
		{
			t.execute(r#"
				INSERT OR REPLACE INTO REQUIRES (recipe_unique_name, item_type, item_count)
				VALUES (?1, ?2, ?3)"#,
				params![
					recipe.unique_name,
					ingredient.item_type,
					ingredient.item_count])?;
		}
	}
	t.commit()?;
	
	// let t = rusqlite::Transaction::new(db, rusqlite::TransactionBehavior::Deferred)?;
	// for recipe in recipes
	// {
	// 	if let Some(common_name) = bp_common_name(&t, &recipe.result_type)?
	// 	{
	// 		dbg!(&common_name);
	// 		t.execute(r#"
	// 			INSERT OR REPLACE INTO BLUEPRINT (unique_name, name)
	// 				VALUES (?1, ?2)"#,
	// 			params![
	// 				recipe.unique_name,
	// 				common_name])?;
	// 		for ingredient in recipe.ingredients
	// 		{
	// 			dbg!(&ingredient);
	// 			if let Some(common_name) = db::common_name(&t, &ingredient.item_type)?
	// 			{
	// 				t.execute(r#"
	// 				INSERT OR REPLACE INTO BLUEPRINT (unique_name, name)
	// 					VALUES (?1, ?2)"#,
	// 					params![
	// 						ingredient.item_type,
	// 						common_name])?;
	// 			}
	// 		}
	// 	};
	// }
	// t.commit()?;
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
	for warframe in super::warframes::parse_from_file(&cache_dir.join(warframe_endpoint)).unwrap()
	{
		t.execute(r#"
		INSERT OR REPLACE INTO WARFRAME (unique_name, name)
		VALUES (?1, ?2)"#,
		params![
			warframe.unique_name,
			warframe.name])?;
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
		params![
			weapon.unique_name,
			weapon.name])?;
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
		params![
			relic.unique_name,
			relic.name])?;
		for reward in relic.relic_rewards
		{
			let reward_name: String = reward.reward_name.split("/")
				.filter(|c|*c != "StoreItems")
				.collect::<Vec<_>>()
				.join("/");
			t.execute(r#"
				INSERT OR REPLACE INTO RELIC_REWARD (relic, name, rarity)
				VALUES (?1, ?2, ?3)"#,
				params![
					relic.name,
					reward_name,
					reward.rarity.as_str()])?;
		}
	}
	t.commit()?;
	Ok(())
}

pub fn resources(cache_dir: &Path, endpoints: &[String], db: &mut Connection) -> rusqlite::Result<()>
{
	let resource_endpoint = endpoints.iter()
		.find(|e|e.contains("Resource"))
		.expect("No resource endpoint found");

	let resource_path = cache_dir.join(resource_endpoint);
	if !resource_path.exists()
	{
		let manifest = super::live::load_manifest(&resource_endpoint).unwrap();
		std::fs::write(&resource_path, &manifest).unwrap();
	}

	let t = rusqlite::Transaction::new(db, rusqlite::TransactionBehavior::Deferred)?;
	for resource in super::resources::parse_from_file(&cache_dir.join(resource_endpoint)).unwrap()
	{
		t.execute(r#"
		INSERT OR REPLACE INTO RESOURCE (unique_name, name)
		VALUES (?1, ?2)"#,
		params![
			resource.unique_name,
			resource.name])?;
	}
	t.commit()?;
	Ok(())
}