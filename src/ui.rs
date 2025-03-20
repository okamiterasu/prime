use std::collections::HashMap;
use std::path::PathBuf;

use crate::cache;
use crate::Data;
use crate::item_view::ItemView;
use crate::relic::Rarity;
use crate::structures::{Count, UniqueName};
use crate::Tracked;

use eframe::egui;
use egui::Ui;
use egui::Color32;

pub struct App
{
	db: Data,
	tracked: Vec<Tracked>,
	owned: HashMap<UniqueName, u32>,
	add_search: String,
	to_remove: Option<usize>,
	cache_dir: PathBuf
}

impl App
{
	pub fn with_state(
		db: Data,
		tracked: Vec<Tracked>,
		owned: HashMap<UniqueName, u32>,
		cache_dir: PathBuf) -> Self
	{
		Self
		{
			db,
			tracked,
			owned,
			add_search: String::new(),
			to_remove: None,
			cache_dir
		}
	}
}

impl eframe::App for App
{
	fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>)
	{
		let tracked = std::mem::take(&mut self.tracked);
		let owned = std::mem::take(&mut self.owned);
		cache::save_state(
			&self.cache_dir.join("tracked.json"),
			tracked,
			owned).unwrap();
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
	{
		if let Some(i)=self.to_remove.take(){self.tracked.remove(i);}
		ctx.set_visuals(egui::style::Visuals::dark());
		egui::CentralPanel::default().show(ctx, |ui|
		{
			header(ui, &mut self.add_search, &self.db, &mut self.tracked);
			egui::Grid::new("").show(ui, |ui|
			{
				for (i, tracked) in self.tracked.iter().enumerate()
				{
					item(ui, tracked, i, &mut self.owned, &mut self.to_remove);

					if i%7 == 6
					{
						ui.end_row()
					}
				}
			})
		});
	}
}

fn header(
	ui: &mut Ui,
	add_search: &mut String,
	db: &Data,
	tracked: &mut Vec<Tracked>)
{
	ui.heading("Recipe Tracker");
	ui.horizontal(|ui|
	{
		ui.label("Add Item");
		ui.text_edit_singleline(add_search);
		if ui.button("Add").clicked()
		{
			if let Some(unique_name) = db.resource_unique_name(add_search.as_str())
			{
				if let Ok(t) = Tracked::new(db, unique_name)
				{
					tracked.push(t);
				}
				
			}
			add_search.clear();
			tracked.sort_by(|a, b| a.common_name.cmp(&b.common_name));
		}
	});
}

fn item(
	ui: &mut Ui,
	tracked: &Tracked,
	i: usize,
	owned_components: &mut HashMap<UniqueName, u32>,
	to_remove: &mut Option<usize>)
{
	let common_name = tracked.common_name.clone();
	ui.group(|ui|
	{
		ui.vertical(|ui|
		{
			ui.horizontal(|ui|
			{
				if ui.button("Del").clicked()
				{
					*to_remove = Some(i)
				};
				ui.heading(common_name.as_str());
			});
			ui.horizontal(|ui|
			{
				for (recipe, components) in &tracked.recipes
				{
					recipe_group(ui, recipe, components, owned_components);
				}
			});
		});
	});
}

fn recipe_group(
	ui: &mut Ui,
	recipe: &crate::Recipe,
	components: &[(crate::Requirement, Count)],
	owned_components: &mut HashMap<UniqueName, u32>)
{
	ui.vertical(|ui|
	{
		component_group(
			ui,
			owned_components,
			recipe,
			1.into());
		for (component, required) in components
		{
			component_group(
				ui,
				owned_components,
				component,
				required.to_owned());
		}
	});
}
fn component_group(
	ui: &mut Ui,
	owned_components: &mut HashMap<UniqueName, u32>,
	item: impl ItemView,
	required: Count)
{
	let owned = owned_components.entry(item.unique_name())
		.or_default();
	let fullfilled = *owned >= required.to_u32();
	let color = fullfilled.then_some(Color32::BLACK)
		.unwrap_or_else(||ui.visuals().text_color());

	ui.vertical(|ui|
	{
		ui.horizontal(|ui|
		{
			if ui.button("-").clicked()
			{
				*owned = owned.saturating_sub(1);
			}

			if ui.button("+").clicked()
			{
				*owned += 1;
			}
			
			ui.colored_label(color, format!("{owned} of {required}"));
			ui.colored_label(color, item.common_name().as_str());
			
		});

		// No need to bother showing drop information if we already have it
		if fullfilled
		{
			return
		}

		let active_relics = item.active_relics();
		if !active_relics.is_empty()
		{
			ui.vertical(|ui|
			{
				for relic in active_relics
				{
					let color = match relic.rarity
					{
						Rarity::Common=>Color32::BROWN,
						Rarity::Uncommon=>Color32::GRAY,
						Rarity::Rare=>Color32::GOLD
					};
		
					ui.colored_label(color, relic.name());
				}
			});
		}

		let resurgence_relics = item.resurgence_relics();
		if !resurgence_relics.is_empty()
		{
			ui.label("Resurgence Relics");
			ui.vertical(|ui|
			{
				for relic in resurgence_relics
				{
					let color = match relic.rarity
					{
						Rarity::Common=>Color32::BROWN,
						Rarity::Uncommon=>Color32::GRAY,
						Rarity::Rare=>Color32::GOLD
					};
		
					ui.colored_label(color, relic.name());
				}
			});
		}

		if item.available_from_invasion()
		{
			ui.label("Invasion");
		}
	});
}