use std::path::Path;

use rusqlite::{Connection, Statement, params, OptionalExtension};
use anyhow::{Result, Context};

use crate::cache;

const SCHEMA: &str = r#"
BEGIN TRANSACTION;
DROP TABLE IF EXISTS "REQUIRES";
CREATE TABLE IF NOT EXISTS "REQUIRES" (
	"recipe_unique_name"	TEXT NOT NULL,
	"item_type"	TEXT NOT NULL,
	"item_count"	INT NOT NULL,
	CONSTRAINT "PK" PRIMARY KEY("recipe_unique_name","item_type")
);
DROP TABLE IF EXISTS "RELIC_REWARD";
CREATE TABLE IF NOT EXISTS "RELIC_REWARD" (
	"relic"	TEXT NOT NULL,
	"name"	TEXT NOT NULL,
	"rarity"	INT NOT NULL,
	CONSTRAINT "PK" PRIMARY KEY("relic","name")
);
DROP TABLE IF EXISTS "ITEM";
CREATE TABLE IF NOT EXISTS "ITEM" (
	"unique_name"	TEXT NOT NULL,
	"name"	TEXT,
	PRIMARY KEY("unique_name")
);
DROP TABLE IF EXISTS "RESOURCE";
CREATE TABLE IF NOT EXISTS "RESOURCE" (
	"unique_name"	TEXT NOT NULL,
	"common_name"	TEXT NOT NULL,
	PRIMARY KEY("unique_name")
);
DROP TABLE IF EXISTS "WARFRAME";
CREATE TABLE IF NOT EXISTS "WARFRAME" (
	"unique_name"	TEXT NOT NULL
);
DROP TABLE IF EXISTS "WEAPON";
CREATE TABLE IF NOT EXISTS "WEAPON" (
	"unique_name"	TEXT NOT NULL
);
DROP TABLE IF EXISTS "ACTIVE_RELIC";
CREATE TABLE IF NOT EXISTS "ACTIVE_RELIC" (
	"unique_name"	TEXT NOT NULL,
	PRIMARY KEY("unique_name")
);
DROP TABLE IF EXISTS "RECIPE";
CREATE TABLE IF NOT EXISTS "RECIPE" (
	"unique_name"	TEXT NOT NULL,
	"result_type"	TEXT NOT NULL,
	PRIMARY KEY("unique_name")
);
DROP TABLE IF EXISTS "RELIC";
CREATE TABLE IF NOT EXISTS "RELIC" (
	"unique_name"	TEXT NOT NULL,
	"name"	TEXT,
	PRIMARY KEY("unique_name")
);
DROP TABLE IF EXISTS "RESURGENCE_RELIC";
CREATE TABLE IF NOT EXISTS "RESURGENCE_RELIC" (
	"unique_name"	TEXT NOT NULL,
	PRIMARY KEY("unique_name")
);
COMMIT;"#;

const COMPONENTS: &str = r#"
SELECT RESOURCE.common_name, REQUIRES.item_type, REQUIRES.item_count
	FROM REQUIRES
		INNER JOIN RESOURCE
			ON REQUIRES.item_type = RESOURCE.unique_name
	WHERE REQUIRES.recipe_unique_name = ?
	ORDER BY
		item_count ASC,
		REQUIRES.item_type ASC"#;

const ITEM_COMMON_NAME: &str = r#"
SELECT name
FROM ITEM
	WHERE unique_name = ?"#;

const ITEM_UNIQUE_NAME: &str = r#"
SELECT unique_name
	FROM ITEM
	WHERE name = ?"#;

const RESOURCE_COMMON_NAME: &str = r#"
SELECT common_name
FROM RESOURCE
	WHERE unique_name = ?"#;

const RESOURCE_UNIQUE_NAME: &str = r#"
SELECT unique_name
	FROM RESOURCE
	WHERE common_name = ?"#;

const HOW_MANY_NEEDED: &str = r#"
SELECT REQUIRES.item_count
	FROM REQUIRES
	WHERE
		REQUIRES.recipe_unique_name = ?1 AND
		REQUIRES.item_type = ?2"#;

const ACTIVE_COMPONENT_RELICS: &str = r#"
SELECT DISTINCT RELIC.name, RELIC_REWARD.rarity
	FROM RELIC_REWARD
		INNER JOIN RELIC
			ON RELIC_REWARD.relic = RELIC.unique_name
		INNER JOIN ACTIVE_RELIC
			ON RELIC.name = ACTIVE_RELIC.unique_name
	WHERE
		RELIC_REWARD.name = ?1"#;

const ACTIVE_RECIPE_RELICS: &str = r#"
SELECT DISTINCT RELIC.name, RELIC_REWARD.rarity
	FROM RECIPE
		INNER JOIN RELIC_REWARD
			ON RECIPE.unique_name = RELIC_REWARD.name
		INNER JOIN RELIC
			ON RELIC_REWARD.relic = RELIC.unique_name
		INNER JOIN ACTIVE_RELIC
			ON RELIC.name = ACTIVE_RELIC.unique_name
	WHERE
		RECIPE.unique_name = ?1"#;

const RESURGENCE_COMPONENT_RELICS: &str = r#"
SELECT DISTINCT RELIC.name, RELIC_REWARD.rarity
	FROM RELIC_REWARD
		INNER JOIN RELIC
			ON RELIC_REWARD.relic = RELIC.unique_name
		INNER JOIN RESURGENCE_RELIC
			ON RELIC.name = RESURGENCE_RELIC.unique_name
	WHERE
		RELIC_REWARD.name = ?1"#;

const RESURGENCE_RECIPE_RELICS: &str = r#"
SELECT DISTINCT RELIC.name, RELIC_REWARD.rarity
	FROM RECIPE
		INNER JOIN RELIC_REWARD
			ON RECIPE.unique_name = RELIC_REWARD.name
		INNER JOIN RELIC
			ON RELIC_REWARD.relic = RELIC.unique_name
		INNER JOIN RESURGENCE_RELIC
			ON RELIC.name = RESURGENCE_RELIC.unique_name
	WHERE
		RECIPE.unique_name = ?1"#;

const RECIPE: &str = r#"
SELECT unique_name
	FROM RECIPE
	WHERE result_type = ?"#;

pub(crate) struct Database
{
	_conn: &'static Connection,
	components: Statement<'static>,
	item_common_name: Statement<'static>,
	item_unique_name: Statement<'static>,
	resource_common_name: Statement<'static>,
	_resource_unique_name: Statement<'static>,
	how_many_needed: Statement<'static>,
	active_component_relics: Statement<'static>,
	active_recipe_relics: Statement<'static>,
	resurgence_component_relics: Statement<'static>,
	resurgence_recipe_relics: Statement<'static>,
	recipe: Statement<'static>,
}

impl Database
{
	pub(crate) fn open(cache_dir: &Path, db_filename: impl AsRef<Path>) -> Result<Self>
	{
		let db_path = cache_dir.join(db_filename);
		let conn = Connection::open(db_path)?;
		let _conn = Box::leak(Box::new(conn));
		let components = _conn.prepare(COMPONENTS)?;
		let item_common_name = _conn.prepare(ITEM_COMMON_NAME)?;
		let item_unique_name = _conn.prepare(ITEM_UNIQUE_NAME)?;
		let resource_common_name = _conn.prepare(RESOURCE_COMMON_NAME)?;
		let resource_unique_name = _conn.prepare(RESOURCE_UNIQUE_NAME)?;
		let how_many_needed = _conn.prepare(HOW_MANY_NEEDED)?;
		let active_component_relics = _conn.prepare(ACTIVE_COMPONENT_RELICS)?;
		let active_recipe_relics = _conn.prepare(ACTIVE_RECIPE_RELICS)?;
		let resurgence_component_relics = _conn.prepare(RESURGENCE_COMPONENT_RELICS)?;
		let resurgence_recipe_relics = _conn.prepare(RESURGENCE_RECIPE_RELICS)?;
		let recipe = _conn.prepare(RECIPE)?;
		Ok(Self{
			_conn,
			components,
			item_common_name,
			item_unique_name,
			resource_common_name,
			_resource_unique_name: resource_unique_name,
			how_many_needed,
			active_component_relics,
			active_recipe_relics,
			resurgence_component_relics,
			resurgence_recipe_relics,
			recipe})
	}

	pub(crate) fn create(cache_dir: &Path, db_filename: impl AsRef<Path>) -> Result<Self>
	{
		let db_path = cache_dir.join(&db_filename);
		let mut conn = Connection::open(db_path)?;
		conn.execute_batch(SCHEMA)?;
		Database::populate(&mut conn, cache_dir)
			.context("Pupulating Database")?;
		Database::open(cache_dir, db_filename)
			.context("Opening Database")
	}

	pub(crate) fn populate(conn: &mut Connection, cache_dir: &Path) -> Result<()>
	{
		let index = cache::load_index(&cache_dir.join("index_en.txt"))
			.context("loading index")?;
		let t = conn.transaction()?;
		{
			let mut item = t.prepare("INSERT OR IGNORE INTO ITEM(unique_name, name) VALUES (?1, ?2)")?;
			// warframes
			let mut wf = t.prepare("INSERT OR IGNORE INTO WARFRAME(unique_name) VALUES (?1)")?;
			for warframe in cache::load_warframes(cache_dir, &index["ExportWarframes_en.json"])
				.context("loading warframe manifest")?
			{
				let unique_name = &warframe.unique_name;
				let common_name = warframe.name.strip_prefix("<ARCHWING> ")
					.unwrap_or(&warframe.name);
				item.execute([unique_name, common_name])
					.context(format!("Adding item to db: {common_name}"))?;
				wf.execute([unique_name])
					.context(format!("Adding warframe to db: {common_name}"))?;
			}
			// weapons
			let mut wp = t.prepare("INSERT OR IGNORE INTO WEAPON(unique_name) VALUES (?1)")?;
			for weapon in cache::load_weapons(cache_dir, &index["ExportWeapons_en.json"])
				.context("loading weapon manifest")?
			{
				let unique_name = &weapon.unique_name;
				let common_name = &weapon.name;
				item.execute([unique_name, common_name])
					.context(format!("Adding item to db: {common_name}"))?;
				wp.execute([unique_name])
					.context(format!("Adding weapon to db: {common_name}"))?;
			}
			// recipes
			let mut rec = t.prepare("INSERT OR IGNORE INTO RECIPE(unique_name, result_type) VALUES (?1, ?2)")?;
			let mut req = t.prepare("INSERT OR IGNORE INTO REQUIRES(recipe_unique_name, item_type, item_count) VALUES (?1, ?2, ?3)")?;
			for recipe in cache::load_recipes(cache_dir, &index["ExportRecipes_en.json"])
				.context("loading recipe manifest")?
			{
				let unique_name = &recipe.unique_name;
				let result_type = &recipe.result_type;
				rec.execute([unique_name, result_type])
					.context(format!("Adding recipe to db: {unique_name}"))?;
				for ingredient in recipe.ingredients
				{
					req.execute(params![unique_name, ingredient.item_type, ingredient.item_count])
						.context(format!("Adding ingredient to db: {ingredient:#?}"))?;
				}
			}
			// relics
			let mut rel = t.prepare("INSERT OR IGNORE INTO RELIC(unique_name, name) VALUES (?1, ?2)")?;
			let mut rew = t.prepare("INSERT OR IGNORE INTO RELIC_REWARD(relic, name, rarity) VALUES (?1, ?2, ?3)")?;
			for relic in cache::load_relics(cache_dir, &index["ExportRelicArcane_en.json"])
				.context("loading relic manifest")?
			{
				let unique_name = &relic.unique_name;
				let name = &relic.name;
				rel.execute([unique_name, name])
					.context(format!("Adding relic to db: {unique_name}"))?;
				for reward in relic.relic_rewards
				{
					let reward_name = reward.reward_name
						.split('/')
						.filter(|s|*s != "StoreItems")
						.fold(String::new(), |a, b|a+b+"/");
					let reward_name = reward_name
						.strip_suffix('/')
						.unwrap_or(&reward_name);
					rew.execute(params![unique_name, reward_name, reward.rarity.as_str()])
						.context(format!("Adding relic reward to db: {reward_name}"))?;
				}
			}
			let mut acr = t.prepare("INSERT OR IGNORE INTO ACTIVE_RELIC(unique_name) VALUES (?1)")?;
			for active_relic in cache::active_relics(&cache_dir.join("droptable.html"))
				.context("loading active relic list")?
			{
				acr.execute([&active_relic])
					.context(format!("adding active relic to db: {active_relic}"))?;
			}
			let mut rer = t.prepare("INSERT OR IGNORE INTO RESURGENCE_RELIC(unique_name) VALUES (?1)")?;
			let mut rcn = t.prepare("SELECT name FROM RELIC WHERE unique_name = ?")?;
			for resurgence_relic in cache::resurgence_relics(&cache_dir.join("worldstate.json"))
				.context("loading resurgence relic list")?
			{
				let common_name: String = rcn.query_row([&resurgence_relic], |r|r.get(0))
					.context(format!("querying common name for resurgence relic: {resurgence_relic}"))?;
				rer.execute([&common_name])
					.context(format!("adding resurgence relic to db: {common_name}"))?;
			}

			// resources
			let mut res = t.prepare("INSERT OR IGNORE INTO RESOURCE(unique_name, common_name) VALUES (?1, ?2)")?;
			for resource in cache::load_resources(cache_dir, &index["ExportResources_en.json"])
				.context("loading resource manifest")?
			{
				res.execute([&resource.unique_name, &resource.name])
					.context(format!("adding resource to db: {resource:#?}"))?;
			}
		}
		t.commit()?;
		Ok(())
	}

	pub fn requirements(&mut self, recipe_unique_name: &str) -> Result<Vec<(String, u32)>>
	{
		self.components
			.query([recipe_unique_name])
			.with_context(||format!("Running query for requirements of {recipe_unique_name}"))?
			.mapped(|r|
			{
				let name = r.get(1)?;
				let count = r.get(2)?;
				Ok((name, count))
			})
			.map(|r|r.map_err(|e|e.into()))
			.collect()
	}

	pub fn item_common_name(&mut self, unique_name: &str) -> Result<Option<String>>
	{
		self.item_common_name
			.query_row([unique_name], |r|r.get(0))
			.optional()
			.with_context(||format!("Running query for common name of {unique_name}"))
	}

	pub fn item_unique_name(&mut self, common_name: &str) -> Result<String>
	{
		self.item_unique_name
			.query_row([common_name], |r|r.get(0))
			.with_context(||format!("Running query for unique name of {common_name}"))
	}

	pub fn resource_common_name(&mut self, unique_name: &str) -> Result<Option<String>>
	{
		self.resource_common_name
			.query_row([unique_name], |r|r.get(0))
			.optional()
			.with_context(||format!("Running query for common name of {unique_name}"))
	}

	pub fn _resource_unique_name(&mut self, common_name: &str) -> Result<String>
	{
		self._resource_unique_name
			.query_row([common_name], |r|r.get(0))
			.with_context(||format!("Running query for unique name of {common_name}"))
	}

	pub fn how_many_needed(&mut self, recipe_unique_name: &str, resource_unique_name: &str) -> Result<u32>
	{
		self.how_many_needed.query_row(
			[recipe_unique_name, resource_unique_name],
			|r|r.get(0))
			.with_context(||format!("Running query for number of '{resource_unique_name}' needed for '{recipe_unique_name}'"))
	}

	pub(crate) fn active_component_relics(&mut self, component_unique_name: &str) -> Result<Vec<crate::Relic>>
	{
		self.active_component_relics
			.query([component_unique_name])?
			.mapped(|r|
			{
				let relic: String = r.get(0)?;
				let rarity: String = r.get(1)?;
				Ok((relic, rarity))
			})
			.flatten()
			.map(|(name, rarity)|crate::Relic::new(&name, &rarity))
			.collect()
	}

	pub(crate) fn active_recipe_relics(&mut self, result_unique_name: &str) -> Result<Vec<crate::Relic>>
	{
		self.active_recipe_relics
			.query([result_unique_name])
			.with_context(||format!("Running query for currently active relics that drop blueprints for {result_unique_name}"))?
			.mapped(|r|
			{
				let relic: String = r.get(0)?;
				let rarity: String = r.get(1)?;
				Ok((relic, rarity))
			})
			.flatten()
			.map(|(name, rarity)|crate::Relic::new(&name, &rarity))
			.collect()
	}

	pub(crate) fn resurgence_component_relics(&mut self, component_unique_name: &str) -> Result<Vec<crate::Relic>>
	{
		self.resurgence_component_relics
			.query([component_unique_name])
			.with_context(||format!("Running query for active resurgence relics that drop {component_unique_name}"))?
			.mapped(|r|
			{
				let relic: String = r.get(0)?;
				let rarity: String = r.get(1)?;
				Ok((relic, rarity))
			})
			.flatten()
			.map(|(name, rarity)|crate::Relic::new(&name, &rarity))
			.collect()
	}

	pub(crate) fn resurgence_recipe_relics(&mut self, result_unique_name: &str) -> Result<Vec<crate::Relic>>
	{
		self.resurgence_recipe_relics
			.query([result_unique_name])
			.with_context(||format!("Running query for active resurgence relics that drop blueprints for {result_unique_name}"))?
			.mapped(|r|
			{
				let relic: String = r.get(0)?;
				let rarity: String = r.get(1)?;
				Ok((relic, rarity))
			})
			.flatten()
			.map(|(name, rarity)|crate::Relic::new(&name, &rarity))
			.collect()
	}

	pub fn recipe(&mut self, result_unique_name: &str) -> Result<Option<String>>
	{
		self.recipe
			.query_row([result_unique_name],|r|r.get(0))
			.optional()
			.with_context(||format!("Running query for a recipe that create {result_unique_name}"))
	}

	pub fn recipes(&mut self, result_unique_name: &str) -> Result<Vec<String>>
	{
		self.recipe
			.query_map([result_unique_name], |r|r.get(0))
			.with_context(||format!("Running query for recipes that create {result_unique_name}"))?
			.map(|r|r.map_err(|e|e.into()))
			.collect()
	}
}