use std::path::PathBuf;
use std::sync::Arc;

use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{Widget, WidgetExt, Data, Lens, Env, Color, Key, AppDelegate, Selector, lens, LensExt};
use druid::im::{Vector};
use rusqlite::Connection;

use super::{relics, db};

const BRONZE: Color = Color::rgb8(0xB0, 0x65, 0x00);
const SILVER: Color = Color::rgb8(0xC0, 0xC0, 0xC0);
const GOLD: Color = Color::rgb8(0xFF, 0xD7, 0x00);

const TEXT_COLOR: Key<Color> = Key::new("color");

// fn des_vector<'de, D, O>(deserializer: D) -> Result<Vector<O>, D::Error>
// where
// 	D: Deserializer<'de>
// {
// 	deserializer.deserialize_seq();
// }

#[derive(Clone, Data, Lens)]
pub struct State
{
	#[data(ignore)]
	pub db: Arc<Connection>,
	#[data(ignore)]
	pub tracked_path: PathBuf,
	pub tracked_recipes: Vector<Tracked>,
	text: String
}

impl State
{
	pub fn new(
		db: Connection,
		tracked_recipes: Vector<Tracked>,
		tracked_path: impl Into<PathBuf>) -> Self
	{
		Self
		{
			db: Arc::new(db),
			tracked_recipes,
			tracked_path: tracked_path.into(),
			text: Default::default()
		}
	}
}

#[derive(Eq, PartialEq, Clone, Data, Lens, Default, Debug)]
pub struct Tracked
{
	pub common_name: Option<String>,
	pub unique_name: String,
	pub recipe: Component,
	pub requires: Vector<Component>
}

impl Tracked
{
	fn new(db: &Connection, common_name: String) -> rusqlite::Result<Self>
	{
		let unique_name = db::unique_name(db, &common_name)?;
		let recipe_unique_name = db::recipe(db, &unique_name)?;
		let requires = db::requirements(db, &recipe_unique_name)?
			.into_iter()
			.flat_map(|r|Component::new(db, &recipe_unique_name, r.1, &common_name, false))
			.collect();
		let tracked = Self
		{
			recipe: Component::new(db, &unique_name, recipe_unique_name, &common_name, true)?,
			common_name: Some(common_name),
			unique_name,
			requires
		};
		Ok(tracked)
	}
}

#[derive(Eq, PartialEq,Clone, Data, Lens, Default, Debug)]
pub struct Component
{
	pub common_name: Option<String>,
	pub unique_name: String,
	pub count: u32,
	pub owned: u32,
	pub active_relics: Vector<(String, relics::Rarity)>,
	pub resurgence_relics: Vector<(String, relics::Rarity)>
}
impl Component
{
	pub fn new(db: &Connection, recipe_unique_name: &str, unique_name: String, result_common_name: &str, main_bp: bool) -> rusqlite::Result<Self>
	{
		let common_name;
		let count;
		let active_relics;
		let resurgence_relics;
		if main_bp
		{
			common_name = Some("BLUEPRINT".to_string());
			count = 1;
		}
		else
		{
			common_name = db::common_name(db, &unique_name)?
				.as_ref()
				.map(|n|n.trim_start_matches(result_common_name))
				.map(|n|n.trim_start())
				.map(|n|n.to_string());
			count = db::how_many_needed(db, &recipe_unique_name, &unique_name).unwrap();
		}
		active_relics = db::active_relics(db, &unique_name)?.into();
		resurgence_relics = db::resurgence_relics(db, &unique_name)?.into();
		let com = Self
		{
			common_name,
			unique_name,
			count,
			owned: 0,
			active_relics,
			resurgence_relics,
		};
		Ok(com)
	}
}

pub fn builder() -> impl Widget<State>
{
	let mut root = Flex::column();
	let mut header = Flex::row()
		.with_child(Label::new("Tracked Recipes"))
		.with_flex_spacer(1.0);
	let add = Flex::row()
		.with_child(TextBox::new().lens(State::text))
		.with_child(Button::new("Add")
			.on_click(|_, state: &mut State, _|{
				let common_name = state.text.to_ascii_uppercase();
				let db = state.db.as_ref();
				let tracked = Tracked::new(&db, common_name.clone()).unwrap();
				state.tracked_recipes.push_back(tracked);}));
	header.add_child(add);
	root.add_child(header);
	root.add_default_spacer();
	let tracked = Scroll::new(List::new(tracked).horizontal())
		.lens(State::tracked_recipes);
	root.add_child(tracked);
	root
}

fn tracked() -> impl Widget<Tracked>
{
	fn greyout(env: &mut druid::Env, data: &Tracked)
	{
		let color = if data.recipe.owned < data.recipe.count
		{
			Color::WHITE
		}
		else
		{
			Color::GRAY
		};
		env.set(TEXT_COLOR, color);
	}
	let mut root = Flex::column();
	let header = Flex::row()
		.with_flex_child(
			Label::dynamic(|data: &Tracked, _|data.common_name.clone().unwrap_or(data.unique_name.clone()))
				.with_text_size(24.0), 1.0);
	root.add_child(header.align_left());
	root.add_default_spacer();
	
	{
		let mut blueprint = Flex::column();
		let header = Flex::row()
			.with_child(
			Label::dynamic(|data: &Tracked, _|
					data.recipe.common_name
						.clone()
						.unwrap_or(data.unique_name.clone()))
				.with_text_size(20.0)
				.with_text_color(TEXT_COLOR)
				.env_scope(greyout)
				.align_left())
				.with_flex_spacer(1.0)
				.with_child(Button::new("-").on_click(|_, t: &mut Tracked, _|t.recipe.owned = t.recipe.owned.saturating_sub(1)))
				.with_child(Button::new("+").on_click(|_, t: &mut Tracked, _|t.recipe.owned = t.recipe.owned.saturating_add(1)));
		blueprint.add_child(header);
		let requires = Flex::row()
			.with_flex_child(
				Label::new("Requires:")
					.with_text_color(TEXT_COLOR)
					.env_scope(greyout)
					.align_right(), 1.0)
			.with_default_spacer()
			.with_child(
				Label::dynamic(|data: &Tracked, _|data.recipe.count.to_string())
					.with_text_color(TEXT_COLOR)
					.env_scope(greyout)
					.align_right());
		blueprint.add_child(requires);
		let have = Flex::row()
			.with_flex_child(
				Label::new("Have:")
					.with_text_color(TEXT_COLOR)
					.env_scope(greyout)
					.align_right(), 1.0)
			.with_default_spacer()
			.with_child(
				Label::dynamic(|data: &Tracked, _|data.recipe.owned.to_string())
					.with_text_color(TEXT_COLOR)
					.env_scope(greyout)
					.align_right());
		blueprint.add_child(have);

		{
			let mut active_relics = Flex::column();
			let lens = lens!(Tracked, recipe).then(lens!(Component, active_relics));
			let list = List::new(||{
				Label::dynamic(|d: &(String, relics::Rarity), _|{d.0.clone()})
					.with_text_color(TEXT_COLOR)
					.env_scope(|e, d|{
						use relics::Rarity;
						let color = match d.1
						{
							Rarity::COMMON=>Color::OLIVE,
							Rarity::UNCOMMON=>Color::SILVER,
							Rarity::RARE=>Color::YELLOW
						};
						e.set(TEXT_COLOR, color);
					}).align_right().expand_width()}).lens(lens);
			active_relics.add_child(list);
			blueprint.add_child(active_relics);
			blueprint.add_default_spacer();
		}
	
		{
			let mut resurgence_relics = Flex::column();
			let lens = lens!(Tracked, recipe).then(lens!(Component, resurgence_relics));
			let list = List::new(||{
				Label::dynamic(|d: &(String, relics::Rarity), _|{d.0.clone()})
					.with_text_color(TEXT_COLOR)
					.env_scope(|e, d|{
						use relics::Rarity;
						let color = match d.1
						{
							Rarity::COMMON=>BRONZE,
							Rarity::UNCOMMON=>SILVER,
							Rarity::RARE=>GOLD
						};
						e.set(TEXT_COLOR, color);})
					.align_right()
					.expand_width()})
				.lens(lens);
			resurgence_relics.add_child(list);
			blueprint.add_child(resurgence_relics);
			blueprint.add_default_spacer();
		}

		root.add_child(blueprint);
	}

	let requires = List::new(component)
		.lens(Tracked::requires);
	root.add_child(requires);
	root.add_child(
		Button::new("🗑")
			.on_click(|ctx, t: &mut Tracked, _|
			{
				ctx.submit_command(
					UNTRACK_SELECTOR
						.with(t.common_name.clone().unwrap()))}));
	root.border(Color::WHITE, 1.0).rounded(5.0).fix_width(200.0)
}

fn component() -> impl Widget<Component>
{
	fn greyout(env: &mut druid::Env, data: &Component)
	{
		let color = if data.owned < data.count
		{
			Color::WHITE
		}
		else
		{
			Color::GRAY
		};
		env.set(TEXT_COLOR, color);
	}
	let mut root = Flex::column();
	let header = Flex::row()
		.with_child(
			Label::dynamic(|data: &Component, _|
				data.common_name.clone()
				.unwrap_or(data.unique_name.clone()))
			.with_text_size(20.0)
			.with_text_color(TEXT_COLOR)
			.env_scope(greyout)
			.align_left())
		.with_flex_spacer(1.0)
		.with_child(Button::new("-").on_click(|_, c: &mut Component, _|c.owned = c.owned.saturating_sub(1)))
		.with_child(Button::new("+").on_click(|_, c: &mut Component, _|c.owned = c.owned.saturating_add(1)));
	root.add_child(header);
	let requires = Flex::row()
		.with_flex_child(
			Label::new("Requires:")
				.with_text_color(TEXT_COLOR)
				.env_scope(greyout)
				.align_right(), 1.0)
		.with_default_spacer()
		.with_child(
			Label::dynamic(|data: &Component, _|data.count.to_string())
				.with_text_color(TEXT_COLOR)
				.env_scope(greyout)
				.align_right());
	root.add_child(requires);
	let have = Flex::row()
		.with_flex_child(
			Label::new("Have:")
				.with_text_color(TEXT_COLOR)
				.env_scope(greyout)
				.align_right(), 1.0)
		.with_default_spacer()
		.with_child(
			Label::dynamic(|data: &Component, _|data.owned.to_string())
				.with_text_color(TEXT_COLOR)
				.env_scope(greyout)
				.align_right());
	root.add_child(have);

	{
		let mut active_relics = Flex::column();
		let list = List::new(||{
			Label::dynamic(|d: &(String, relics::Rarity), _|{d.0.clone()})
				.with_text_color(TEXT_COLOR)
				.env_scope(|e, d|{
					use relics::Rarity;
					let color = match d.1
					{
						Rarity::COMMON=>Color::OLIVE,
						Rarity::UNCOMMON=>Color::SILVER,
						Rarity::RARE=>Color::YELLOW
					};
					e.set(TEXT_COLOR, color);
				}).align_right().expand_width()}).lens(Component::active_relics);
		active_relics.add_child(list);
		root.add_child(active_relics);
		root.add_default_spacer();
	}

	{
		let mut resurgence_relics = Flex::column();
		let list = List::new(||{
			Label::dynamic(|d: &(String, relics::Rarity), _|{d.0.clone()})
				.with_text_color(TEXT_COLOR)
				.env_scope(|e, d|{
					use relics::Rarity;
					let color = match d.1
					{
						Rarity::COMMON=>BRONZE,
						Rarity::UNCOMMON=>SILVER,
						Rarity::RARE=>GOLD
					};
					e.set(TEXT_COLOR, color);
				}).align_right().expand_width()}).lens(Component::resurgence_relics);
		resurgence_relics.add_child(list);
		root.add_child(resurgence_relics);
		root.add_default_spacer();
	}

	root.add_default_spacer();
	root
}

pub struct Delegate;
const UNTRACK_SELECTOR: Selector<String> = Selector::new("Untrack");

impl AppDelegate<State> for Delegate
{
	fn command(
		&mut self,
		_ctx: &mut druid::DelegateCtx,
		_target: druid::Target,
		cmd: &druid::Command,
		data: &mut State,
		_env: &Env
	) -> druid::Handled {
		use druid::Handled;
		if let Some(name) = cmd.get(UNTRACK_SELECTOR)
		{
			let tracked = &mut data.tracked_recipes;
			let to_remove = tracked.iter()
				.position(|t|t.common_name.as_ref() == Some(name))
				.expect("Recipe not found");
			tracked.remove(to_remove);
			return Handled::Yes
		}

		if cmd.get(druid::commands::CLOSE_WINDOW).is_some()
		{
			super::persistance::save(&data.tracked_path, &data.tracked_recipes).unwrap();
			return Handled::No
		}

		Handled::No
	}
}