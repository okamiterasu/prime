use std::collections::HashMap;
use std::path::PathBuf;

use crate::db::Database;
use crate::cache;
use crate::Tracked;

use eframe::egui;
use egui::Ui;

pub(crate) struct App
{
	db: Database,
	tracked: Vec<Tracked>,
	owned: HashMap<String, u32>,
	add_search: String,
	to_remove: Option<usize>,
	cache_dir: PathBuf
}

impl App
{
	pub(crate) fn with_state(
		db: Database,
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
	fn on_exit(&mut self, _gl: &eframe::glow::Context)
	{
		cache::save_state(
			&self.cache_dir.join("tracked.json"),
			&self.tracked,
			&self.owned).unwrap();
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
	{
		ctx.set_visuals(egui::style::Visuals::dark());
		egui::CentralPanel::default().show(ctx, |ui|
		{
			ui.heading("Recipe Tracker");
			ui.horizontal(|ui|
			{
				ui.label("Add Item");
				ui.text_edit_singleline(&mut self.add_search);
				if ui.button("Add").clicked()
				{
					if let Ok(unique_name) = self.db.item_unique_name(&self.add_search)
					{
						let t = Tracked::new(&mut self.db, unique_name).unwrap();
						self.tracked.push(t);
					}
					self.add_search.clear();
					self.tracked.sort_by(|a, b|a.common_name.cmp(&b.common_name));
				}
			});
			egui::Grid::new("").show(ui, |ui|
			{
				if let Some(i)=self.to_remove.take(){self.tracked.remove(i);}
				for (i, tracked) in self.tracked.iter().enumerate()
				{
					recipe_group(
						ui,
						tracked,
						i,
						&mut self.owned,
						&mut self.to_remove);
					if i%7==6{ui.end_row()}
				}
			});
		});
	}
}

fn recipe_group(
	ui: &mut Ui,
	tracked: &Tracked,
	i: usize,
	owned_components: &mut HashMap<String, u32>,
	to_remove: &mut Option<usize>) -> egui::InnerResponse<()>
{
	let unique_name = &tracked.unique_name;
	let common_name = tracked.common_name.as_deref();
	ui.group(|ui|
	{
		ui.vertical(|ui|
		{
			if ui.button("Del").clicked() {*to_remove = Some(i)}
			ui.heading(common_name.unwrap_or(unique_name));
			let recipe_unique_name = &tracked.recipe.unique_name;
			let recipe_common_name = tracked.recipe.common_name.as_deref();
			component_group(
				ui,
				recipe_unique_name,
				recipe_common_name,
				owned_components,
				1);
			for component in &tracked.components
			{
				let component_unique_name = &component.unique_name;
				let component_common_name = component.common_name.as_deref();
				let required = component.count;
				component_group(
					ui,
					component_unique_name,
					component_common_name,
					owned_components,
					required);
			}
		});
	})
}

fn component_group(
	ui: &mut Ui,
	unique_name: &str,
	common_name: Option<&str>,
	owned_components: &mut HashMap<String, u32>,
	required: u32) -> egui::InnerResponse<()>
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

		let color = if *owned>=1 {
			egui::Color32::BLACK
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
	})
}