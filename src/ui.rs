use std::path::PathBuf;

use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{Widget, WidgetExt, Data, Lens, Env, Color, Key, AppDelegate, Selector};
use druid::im::{Vector};

use super::{relics, db};

const TEXT_COLOR: Key<Color> = Key::new("color");

// fn des_vector<'de, D, O>(deserializer: D) -> Result<Vector<O>, D::Error>
// where
// 	D: Deserializer<'de>
// {
// 	deserializer.deserialize_seq();
// }

#[derive(Clone, Data, Lens, Default)]
pub struct State
{
	pub tracked_recipes: Vector<Tracked>,
	#[data(ignore)]
	pub db_path: PathBuf,
	#[data(ignore)]
	pub tracked_path: PathBuf,
	text: String
}

#[derive(Eq, PartialEq, Clone, Data, Lens, Default)]
pub struct Tracked
{
	pub common_name: Option<String>,
	pub unique_name: String,
	pub recipe: (String, u32),
	pub requires: Vector<Component>
}

impl Tracked
{
	fn new(common_name: Option<String>, unique_name: String, recipe_unique_name: String) -> Self
	{
		Self{
			common_name,
			unique_name,
			recipe: (recipe_unique_name, 0),
			requires: Default::default()
		}
	}
}

#[derive(Eq, PartialEq,Clone, Data, Lens, Default, Debug)]
pub struct Component
{
	pub common_name: Option<String>,
	pub unique_name: String,
	pub count: u32,
	pub owned: u32,
	pub relics: Vector<(String, super::relics::Rarity)>
}
impl Component
{
	fn new(name: Option<String>, unique_name: String, count: u32) -> Self
	{
		Self{common_name: name, count, unique_name,..Default::default()}
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
				let mut db = rusqlite::Connection::open(&state.db_path).unwrap();
				let (unique_name, unique_recipe_name) = db::find_unique_with_recipe(&db, &common_name).unwrap();
				// let unique_name = db::unique_name_main(&mut db, &common_name).unwrap();
				let requirements = super::db::requirements(&mut db, &unique_recipe_name).unwrap();
				let mut tracked = Tracked::new(Some(common_name.clone()), unique_name, unique_recipe_name);
				for r in requirements
				{
					let common_name = r.0
						.as_ref()
						.map(|r|r.trim_start_matches(&common_name))
						.map(|n|n.trim_start())
						.map(|r|r.to_owned());
					let relics = super::db::relics(&mut db, &r.1).unwrap();
					let mut com = Component::new(common_name, r.1, r.2);
					com.relics.extend(relics);
					tracked.requires.push_back(com);
				}
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
	let mut root = Flex::column();
	let header = Flex::row()
		.with_flex_child(
			Label::dynamic(|data: &Tracked, _|data.common_name.clone().unwrap_or(data.unique_name.clone()))
				.with_text_size(24.0), 1.0);
	root.add_child(header.align_left());
	root.add_default_spacer();
	let requires = List::new(component)
		.lens(Tracked::requires);
	root.add_child(requires);
	root.add_child(
		Button::new("Del")
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
	let mut header = Flex::row();
	header.add_flex_child(
		Label::dynamic(|data: &Component, _|data.common_name.clone().unwrap_or(data.unique_name.clone()))
			.with_text_size(20.0)
			.with_text_color(TEXT_COLOR)
			.env_scope(greyout)
			.align_left(), 1.0);
	root.add_child(header);
	let buttons = Flex::row()	
		.with_child(Button::new("-").on_click(|_, c: &mut Component, _|c.owned = c.owned.saturating_sub(1)))
		.with_child(Button::new("+").on_click(|_, c: &mut Component, _|c.owned = c.owned.saturating_add(1)))
		.align_right()
		.expand_width();
	root.add_child(buttons);
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

	let relics = List::new(||{
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
				})})
		.lens(Component::relics);
	root.add_child(relics);

	root.add_default_spacer();
	root
}

pub struct Delegate;
const UNTRACK_SELECTOR: Selector<String> = Selector::new("Untrack");

impl AppDelegate<State> for Delegate
{
	fn command(
		&mut self,
		ctx: &mut druid::DelegateCtx,
		target: druid::Target,
		cmd: &druid::Command,
		data: &mut State,
		env: &Env
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
			// let tracked: Vec<_> = data.tracked_recipes.iter().cloned().collect();
			super::persistance::save(&data.tracked_path, &data.tracked_recipes).unwrap();
			return Handled::No
		}

		Handled::No
	}
}