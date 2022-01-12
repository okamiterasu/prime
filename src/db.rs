use std::collections::HashSet;

use rusqlite::Connection;

use crate::relics;

enum Ternary
{
	True,
	False,
	Whatever
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