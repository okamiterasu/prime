use std::path::{Path};
use rusqlite::{params, Connection};
use crate::{droptable, worldstate};

pub fn recipes(db: &Connection, file_path: &Path) -> rusqlite::Result<()>
{
	let recipes = super::recipes::parse_from_file(file_path).unwrap();
	for recipe in &recipes
	{
		db.execute(r#"
			INSERT OR REPLACE INTO RECIPE (unique_name, result_type)
			VALUES (?1, ?2)"#,
			params![
				recipe.unique_name,
				recipe.result_type])?;		

		for ingredient in &recipe.ingredients
		{
			db.execute(r#"
				INSERT OR REPLACE INTO REQUIRES (recipe_unique_name, item_type, item_count)
				VALUES (?1, ?2, ?3)"#,
				params![
					recipe.unique_name,
					ingredient.item_type,
					ingredient.item_count])?;
		}
	}
	Ok(())
}

pub fn warframes(db: &Connection, file_path: &Path) -> rusqlite::Result<()>
{
	for warframe in super::warframes::parse_from_file(file_path).unwrap()
	{
		db.execute(r#"
		INSERT OR REPLACE INTO WARFRAME (unique_name, name)
		VALUES (?1, ?2)"#,
		params![
			warframe.unique_name,
			warframe.name])?;
	}
	Ok(())
}

pub fn weapons(db: &Connection, file_path: &Path) -> rusqlite::Result<()>
{
	for weapon in super::weapons::parse_from_file(file_path).unwrap()
	{
		if !weapon.name.contains("PRIME")
		{
			continue
		}
		db.execute(r#"
		INSERT OR REPLACE INTO WEAPON (unique_name, name)
		VALUES (?1, ?2)"#,
		params![
			weapon.unique_name,
			weapon.name])?;
	}
	Ok(())
}

pub fn relics(db: &Connection, file_path: &Path) -> rusqlite::Result<()>
{
	let parent = file_path.parent().unwrap();
	let active_relics = droptable::active_relics(&parent.join("drops.html")).unwrap();
	let resurgence_relics = worldstate::resurgence_relics(&parent.join("worldstate.json")).unwrap();
	for relic in super::relics::parse_from_file(&file_path).unwrap()
	{
		let active = active_relics.contains(&relic.name);
		let resurgence = resurgence_relics.contains(&relic.unique_name);
		db.execute(r#"
		INSERT OR REPLACE INTO RELIC (unique_name, name, active, resurgence)
		VALUES (?1, ?2, ?3, ?4)"#,
		params![
			relic.unique_name,
			relic.name,
			active,
			resurgence])?;
		for reward in relic.relic_rewards
		{
			let reward_name: String = reward.reward_name.split("/")
				.filter(|c|*c != "StoreItems")
				.collect::<Vec<_>>()
				.join("/");
			db.execute(r#"
				INSERT OR REPLACE INTO RELIC_REWARD (relic, name, rarity)
				VALUES (?1, ?2, ?3)"#,
				params![
					relic.unique_name,
					reward_name,
					reward.rarity.as_str()])?;
		}
	}
	Ok(())
}

pub fn resources(db: &Connection, file_path: &Path) -> rusqlite::Result<()>
{
	for resource in super::resources::parse_from_file(file_path).unwrap()
	{
		db.execute(r#"
		INSERT OR REPLACE INTO RESOURCE (unique_name, name)
		VALUES (?1, ?2)"#,
		params![
			resource.unique_name,
			resource.name])?;
	}
	Ok(())
}