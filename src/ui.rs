use std::collections::HashMap;
use std::path::PathBuf;

use crate::Rarity;
use crate::Relic;
use crate::Data;
use crate::cache;
use crate::Tracked;

use eframe::egui;
use egui::Ui;
use egui::Color32;

pub(crate) struct App
{
	db: Data,
	tracked: Vec<Tracked>,
	owned: HashMap<String, u32>,
	add_search: String,
	to_remove: Option<usize>,
	cache_dir: PathBuf
}

impl App
{
	pub(crate) fn with_state(
		db: Data,
		tracked: Vec<Tracked>,
		owned: HashMap<String, u32>,
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
		cache::save_state(
			&self.cache_dir.join("tracked.json"),
			&self.tracked,
			&self.owned).unwrap();
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
	{
		if let Some(i)=self.to_remove.take(){self.tracked.remove(i);}
		ctx.set_visuals(egui::style::Visuals::dark());
		egui::CentralPanel::default().show(ctx, |ui|
		{
			header(ui, &mut self.add_search, &mut self.db, &mut self.tracked);
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
	db: &mut Data,
	tracked: &mut Vec<Tracked>) -> egui::InnerResponse<()>
{
	ui.heading("Recipe Tracker");
	ui.horizontal(|ui|
	{
		ui.label("Add Item");
		ui.text_edit_singleline(add_search);
		if ui.button("Add").clicked()
		{
			if let Ok(unique_name) = db.item_unique_name(add_search)
			{
				if let Ok(t) = Tracked::new(db, unique_name)
				{
					tracked.push(t);
				}
				
			}
			add_search.clear();
			tracked.sort_by(|a, b|a.common_name.cmp(&b.common_name));
		}
	})
}

fn item(ui: &mut Ui, tracked: &Tracked, i: usize, owned_components: &mut HashMap<String, u32>, to_remove: &mut Option<usize>) -> egui::InnerResponse<()>
{
	let unique_name = &tracked.unique_name;
	let common_name = tracked.common_name.as_deref();
	ui.group(|ui|
	{
		ui.vertical(|ui|
		{
			ui.horizontal(|ui|
			{
				if ui.button("Del").clicked() {*to_remove = Some(i)};
				ui.heading(common_name.unwrap_or(unique_name));
			});
			ui.horizontal(|ui|
			{
				for (recipe, components) in &tracked.recipes
				{
					recipe_group(ui, recipe, components, owned_components);
				}
			});
		});
	})
}

fn recipe_group(
	ui: &mut Ui,
	recipe: &crate::Recipe,
	components: &[crate::Component],
	owned_components: &mut HashMap<String, u32>) -> egui::InnerResponse<()>
{
	ui.vertical(|ui|
	{
		let recipe_unique_name = recipe.unique_name.as_str();
		let recipe_common_name = recipe.common_name.as_deref();
		component_group(
			ui,
			recipe_unique_name,
			recipe_common_name,
			owned_components,
			1,
			&recipe.active_relics,
			&recipe.resurgence_relics);
		for component in components
		{
			let component_unique_name = &component.unique_name;
			let component_common_name = component.common_name.as_deref();
			let required = component.count;
			let active_relics = component.recipe.as_ref()
				.map(|r|&r.active_relics)
				.unwrap_or(&component.active_relics);
			let resurgence_relics = component.recipe.as_ref()
				.map(|r|&r.resurgence_relics)
				.unwrap_or(&component.resurgence_relics);
			component_group(
				ui,
				component_unique_name,
				component_common_name,
				owned_components,
				required,
				active_relics,
				resurgence_relics);
		}
	})
}

fn component_group(
	ui: &mut Ui,
	unique_name: &str,
	common_name: Option<&str>,
	owned_components: &mut HashMap<String, u32>,
	required: u32,
	active_relics: &[Relic],
	resurgence_relics: &[Relic]) -> egui::InnerResponse<()>
{
	let mut fullfilled = false;
	ui.vertical(|ui|
	{
		ui.horizontal(|ui|
		{
			let owned = match owned_components.get_mut(unique_name)
			{
				Some(v)=>v,
				None=>owned_components
						.entry(unique_name.to_owned())
						.or_insert(0)
			};
			fullfilled = *owned>=required;

			let color = if fullfilled {
				Color32::BLACK
			} else {
				ui.visuals().text_color()
			};

			if ui.button("-").clicked()
			{
				*owned = owned.saturating_sub(1);
			}

			if ui.button("+").clicked()
			{
				*owned += 1;
			}

			let name = common_name.unwrap_or(unique_name);
			ui.colored_label(color, format!("{} of {}", owned, required));
			ui.colored_label(color, name);
			
		});
		if !active_relics.is_empty() && !fullfilled
		{
			ui.vertical(|ui|
			{
				for relic in active_relics
				{
					let color = match relic.rarity {
						Rarity::COMMON=>Color32::BROWN,
						Rarity::UNCOMMON=>Color32::GRAY,
						Rarity::RARE=>Color32::GOLD
					};
		
					ui.colored_label(color, &relic.name);
				}
			});
		}
		if !resurgence_relics.is_empty() && !fullfilled
		{
			ui.label("Resurgence Relics");
			ui.vertical(|ui|
			{
				for relic in resurgence_relics
				{
					let color = match relic.rarity {
						Rarity::COMMON=>Color32::BROWN,
						Rarity::UNCOMMON=>Color32::GRAY,
						Rarity::RARE=>Color32::GOLD
					};
		
					ui.colored_label(color, &relic.name);
				}
			});
		}
	})
}