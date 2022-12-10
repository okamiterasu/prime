use std::collections::HashMap;
use std::path::PathBuf;

use crate::Rarity;
use crate::Relic;
use crate::Data;
use crate::cache;
use crate::Tracked;
use crate::structures::{CommonName, UniqueName, Count};

use eframe::egui;
use egui::Ui;
use egui::Color32;

pub(crate) struct App
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
	pub(crate) fn with_state(
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
			if let Some(unique_name) = db.item_unique_name(add_search.as_str())
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

fn item(ui: &mut Ui, tracked: &Tracked, i: usize, owned_components: &mut HashMap<UniqueName, u32>, to_remove: &mut Option<usize>) -> egui::InnerResponse<()>
{
	let unique_name = tracked.unique_name.clone();
	let common_name = tracked.common_name.clone();
	ui.group(|ui|
	{
		ui.vertical(|ui|
		{
			ui.horizontal(|ui|
			{
				if ui.button("Del").clicked() {*to_remove = Some(i)};
				ui.heading(common_name.unwrap_or_else(||unique_name.into()).as_str());
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
	owned_components: &mut HashMap<UniqueName, u32>) -> egui::InnerResponse<()>
{
	ui.vertical(|ui|
	{
		let recipe_unique_name = recipe.unique_name.clone();
		let recipe_common_name = recipe.common_name.clone();
		component_group(
			ui,
			recipe_unique_name,
			recipe_common_name,
			owned_components,
			1.into(),
			&recipe.active_relics,
			&recipe.resurgence_relics,
			recipe.available_from_invasion);
		for component in components
		{
			let component_unique_name = component.unique_name.clone();
			let component_common_name = component.common_name.clone();
			let required = component.count.clone();
			let active_relics = component.recipe.as_ref()
				.map(|r|&r.active_relics)
				.unwrap_or(&component.active_relics);
			let resurgence_relics = component.recipe.as_ref()
				.map(|r|&r.resurgence_relics)
				.unwrap_or(&component.resurgence_relics);
			let available_from_invastion = component.available_from_invasion;
			component_group(
				ui,
				component_unique_name,
				component_common_name,
				owned_components,
				required,
				active_relics,
				resurgence_relics,
				available_from_invastion);
		}
	})
}

#[allow(clippy::too_many_arguments)]
fn component_group(
	ui: &mut Ui,
	unique_name: UniqueName,
	common_name: Option<CommonName>,
	owned_components: &mut HashMap<UniqueName, u32>,
	required: Count,
	active_relics: &[Relic],
	resurgence_relics: &[Relic],
	available_from_invastion: bool) -> egui::InnerResponse<()>
{
	let mut fullfilled = false;
	ui.vertical(|ui|
	{
		ui.horizontal(|ui|
		{
			let owned = match owned_components.get_mut(&unique_name)
			{
				Some(v)=>v,
				None=>owned_components
						.entry(unique_name.to_owned())
						.or_insert(0)
			};
			fullfilled = *owned>=required.to_u32();

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

			let name = common_name.unwrap_or_else(||unique_name.into());
			ui.colored_label(color, format!("{} of {}", owned, required));
			ui.colored_label(color, name.as_str());
			
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
		if available_from_invastion && !fullfilled
		{
			ui.label("Invastion");
		}
	})
}