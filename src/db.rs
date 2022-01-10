use rusqlite::{Connection};

use crate::relics;

pub fn find_unique_with_recipe(db: &Connection, common_name: &str) -> rusqlite::Result<(String, String)>
{
	let mut query = db.prepare(r#"
		SELECT T1.unique_name, RECIPE.unique_name
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
			INNER JOIN RECIPE
				ON T1.unique_name = RECIPE.result_type
		WHERE T1.name = ?"#)?;
	let response = query.query_row([common_name], |r|Ok((r.get(0)?, r.get(1)?)))?;
	Ok(response)
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
		
	let response = components.query_map([recipe_unique_name], |r|{
			let common_name = r.get(0)?;
			let unique_name = r.get(1)?;
			let count = r.get(2)?;
			Ok((common_name, unique_name, count))})?
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
	dbg!(common_name);
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

pub fn unique_name_main(db: &Connection, common_name: &str) -> rusqlite::Result<String>
{
	let mut query = db.prepare(r#"
	SELECT T1.unique_name
		FROM
		(
			SELECT WARFRAME.name, WARFRAME.unique_name
				FROM WARFRAME
			UNION
			SELECT WEAPON.name, WEAPON.unique_name
				FROM WEAPON
		) T1
	WHERE T1.name LIKE '%'||?"#)?;
	let response: String = query.query_row([common_name], |r|r.get(0))?;
	Ok(response)
}

pub fn how_many_needed(db: &Connection, recipe_unique_name: &str, resource_unique_name: &str) -> rusqlite::Result<u32>
{
	dbg!(recipe_unique_name, resource_unique_name);
	let mut query = db.prepare(r#"
		SELECT REQUIRES.item_count
			FROM REQUIRES
		WHERE
			REQUIRES.recipe_unique_name = ?1 AND
			REQUIRES.item_type = ?2"#)?;

	let response: u32 = query
		.query_row([recipe_unique_name, resource_unique_name], |r|r.get(0))
		.unwrap();
	Ok(response)
}

pub fn relics(db: &Connection, unique_name: &str) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	dbg!(unique_name);
	let mut relics = Vec::new();
	let component_relics = component_relics(db, unique_name)?;
	dbg!(&component_relics);
	relics.extend(component_relics);
	let recipe_relics = recipe_relics(db, unique_name)?;
	dbg!(&recipe_relics);
	relics.extend(recipe_relics);
	Ok(relics)
}

fn component_relics(db: &Connection, component_unique_name: &str) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	let mut query = db.prepare(r#"
	SELECT RELIC_REWARD.relic, RELIC_REWARD.rarity
		FROM RELIC_REWARD
	WHERE RELIC_REWARD.name = ?"#)?;

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

fn recipe_relics(db: &Connection, result_unique_name: &str) -> rusqlite::Result<Vec<(String, relics::Rarity)>>
{
	let mut query = db.prepare(r#"
	SELECT RELIC_REWARD.relic, RELIC_REWARD.rarity
		FROM RECIPE
			INNER JOIN RELIC_REWARD
				ON RECIPE.unique_name = RELIC_REWARD.name
	WHERE RECIPE.result_type = ?"#)?;

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