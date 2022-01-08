use rusqlite::{params, Connection};

pub fn requirements(db: &mut Connection, item_name: &str) -> rusqlite::Result<Vec<(String, u32)>>
{
	let item_name = item_name.to_ascii_uppercase();
	let mut query = db.prepare(r#"
	SELECT T1.name, T1.unique_name, RECIPE.result_type, REQUIRES.name, REQUIRES.item_count
		FROM
		(
			SELECT WARFRAME.name, WARFRAME.unique_name
				FROM WARFRAME
			UNION
			SELECT WEAPON.name, WEAPON.unique_name
				FROM WEAPON
		) T1
			INNER JOIN RECIPE
					ON T1.unique_name = RECIPE.result_type
				LEFT JOIN REQUIRES
					ON RECIPE.result_type = REQUIRES.result_type
	WHERE T1.name = ?
	ORDER BY
		item_count ASC,
		REQUIRES.name ASC"#)?;

		
	let response = query.query([item_name])?;
	let mut requirements = Vec::new();
	requirements.push(("Blueprint".to_string(), 1));
	requirements.extend(response
		.mapped(|row|
		{
			let name = row.get(3)?;
			let count = row.get(4)?;
			Ok((name, count))
		}).flatten()
		.filter(|r|r.0 != "OrokinCell"));
	Ok(requirements)
}

// pub fn relics(db: &mut Connection, item: &str) -> rusqlite::Result<Vec<(String, super::relics::Rarity)>>
// {
// 	let item_name = item.to_ascii_uppercase();
// 	let mut query = db.prepare(r#"
// 	SELECT RECIPE.result_type, REQUIRES.name, REQUIRES.item_count
// 		FROM
// 			RECIPE
// 			INNER JOIN RELIC_REWARD
// 				ON RELIC_REWARD.name = T1.unique_name
// 		WHERE"#)?;
// 		Ok(Default::default())
// }