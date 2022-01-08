use std::path::PathBuf;

use druid::widget::{Button, Flex, Label, ClipBox, List, Scroll, TextBox, Container};
use druid::{Widget, WidgetExt, WindowDesc, Data, Lens, Env, Color, UnitPoint, Key, AppDelegate, Selector, Command};
use druid::im::{HashMap, Vector};

use rusqlite::{params, Connection};
use serde::{Deserialize, Deserializer, Serialize};

const TEXT_COLOR: Key<Color> = Key::new("color");

// fn des_vector<'de, D, O>(deserializer: D) -> Result<Vector<O>, D::Error>
// where
// 	D: Deserializer<'de>
// {
// 	deserializer.deserialize_seq();
// }

#[derive(Eq, PartialEq,Clone, Data, Lens, Default, Deserialize, Serialize)]
struct Component
{
	name: String,
	count: u32,
	owned: u32,
	#[serde(skip)]
	relics: Vector<String>
}
impl Component
{
	fn new(name: String, count: u32) -> Self
	{
		Self{name, count,..Default::default()}
	}
}


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
				let name = state.text.clone();
				let mut db = rusqlite::Connection::open(&state.db_path).unwrap();
				let requirements = super::db::requirements(&mut db, &name).unwrap();
				if requirements.len() <=1
				{
					eprintln!("{} not found", name);
					return
				}
				let mut tracked = Tracked::new(name);
				for r in requirements
				{
					// let relics = super::db::relics(&mut db, &r.0);
					tracked.requires.push_back(Component::new(r.0, r.1));
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

// ugly hack to deserialize `Vec` into `Vector`
// TODO: find better way
#[derive(Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
struct TrackedHack
{
	name: String,
	requires: Vec<Component>
}

impl From<Tracked> for TrackedHack
{
	fn from(a: Tracked) -> Self {
		Self{name: a.name, requires: a.requires.iter().cloned().collect()}
	}
}

#[derive(Eq, PartialEq, Clone, Data, Lens, Default, Deserialize, Serialize)]
#[serde(from = "TrackedHack", into="TrackedHack")]
pub struct Tracked
{
	name: String,
	requires: Vector<Component>
}

impl Tracked
{
	fn new(name: String) -> Self
	{
		Self{
			name,
			requires: Default::default()
		}
	}
}

impl From<TrackedHack> for Tracked
{
	fn from(a: TrackedHack) -> Self {
		Self{name: a.name, requires: a.requires.into()}
	}
}

fn tracked() -> impl Widget<Tracked>
{
	let mut root = Flex::column();
	let header = Flex::row()
		.with_flex_child(
			Label::dynamic(|data: &Tracked, _|data.name.clone())
				.with_text_size(24.0), 1.0);
	root.add_child(header.align_left());
	root.add_default_spacer();
	let requires = List::new(requires)
		.lens(Tracked::requires);
	root.add_child(requires);
	root.add_child(
		Button::new("Del")
			.on_click(|ctx, t: &mut Tracked, _|
			{
				ctx.submit_command(
					Selector::new("Untrack")
						.with(t.name.clone()))}));
	root.border(Color::WHITE, 1.0).rounded(5.0).fix_width(200.0)
}

fn requires() -> impl Widget<Component>
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
		Label::dynamic(|data: &Component, _|data.name.clone())
			.with_text_size(20.0)
			.with_text_color(TEXT_COLOR)
			.env_scope(greyout)
			.align_left(), 1.0);
	header.add_child(Button::new("-").on_click(|_, c: &mut Component, _|c.owned = c.owned.saturating_sub(1)));
	header.add_child(Button::new("+").on_click(|_, c: &mut Component, _|c.owned = c.owned.saturating_add(1)));
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

	// let relics = List::new(
	// 	||Label::dynamic(|d: &String, _|d.clone()).with_text_color(TEXT_COLOR))
	// 	.lens(Component::relics);
	// root.add_child(relics);

	root.add_default_spacer();
	root
}

pub struct Delegate;

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
		let untrack_selector: Selector<String> = Selector::new("Untrack");
		if let Some(name) = cmd.get(untrack_selector)
		{
			let tracked = &mut data.tracked_recipes;
			let to_remove = tracked.iter()
				.position(|t|t.name == *name)
				.expect("Recipe not found");
			tracked.remove(to_remove);
			return Handled::Yes
		}

		if cmd.get(druid::commands::CLOSE_WINDOW).is_some()
		{
			let tracked: Vec<_> = data.tracked_recipes.iter().cloned().collect();
			let path = &data.tracked_path;
			let mut buf = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
			serde_json::to_writer(&mut buf, &tracked).expect("failed to write json");
			return Handled::No
		}

		Handled::No
	}
}