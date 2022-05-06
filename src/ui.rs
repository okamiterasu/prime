use std::collections::HashMap;
use std::path::PathBuf;

use crate::db::Database;
use crate::cache;
use crate::Tracked;

use eframe::egui;

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
	pub(crate) fn with_state(db: Database, tracked: Vec<Tracked>, owned: HashMap<String, u32>, cache_dir: PathBuf) -> Self
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
					// dbg!(&self.tracked);
				}
			});
			egui::Grid::new("").show(ui, |ui|
			{
				if let Some(i)=self.to_remove.take(){self.tracked.remove(i);}
				for (i, tracked) in self.tracked.iter().enumerate()
				{
					recipe_group(ui, tracked, i, &mut self.owned, &mut self.to_remove);
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
	owned: &mut HashMap<String, u32>,
	to_remove: &mut Option<usize>) -> egui::InnerResponse<()>
{
	ui.group(|ui|
	{
		ui.vertical(|ui|
		{
			if ui.button("Del").clicked()
			{
				*to_remove = Some(i);
			}
			ui.heading(tracked.common_name.as_ref().unwrap_or(&tracked.unique_name));
			ui.horizontal(|ui|
			{
				let owned = match owned.get_mut(&tracked.recipe.unique_name)
				{
					Some(v)=>v,
					None=>owned.entry(tracked.recipe.unique_name.clone()).or_insert(0)
				};
				let color = if *owned>=1 {egui::Color32::BLACK} else {ui.visuals().text_color()};
				if ui.button("-").clicked()
				{
					*owned = owned.saturating_sub(1);
				}
				if ui.button("+").clicked()
				{
					*owned += 1;
				}
				ui.colored_label(color, format!("{} of {}", owned, 1));
				ui.colored_label(color, tracked.recipe.common_name.as_ref().unwrap_or(&tracked.recipe.unique_name));
			});
			for component in &tracked.components
			{
				ui.horizontal(|ui|
				{
					let owned = match owned.get_mut(&component.unique_name)
					{
						Some(v)=>v,
						None=>owned.entry(component.unique_name.clone()).or_insert(0)
					};
					let color = if *owned>=component.count {egui::Color32::BLACK} else {ui.visuals().text_color()};
					if ui.button("-").clicked()
					{
						*owned = owned.saturating_sub(1);
					}
					if ui.button("+").clicked()
					{
						*owned += 1;
					}
					ui.colored_label(color, format!("{} of {}", owned, component.count));
					ui.colored_label(color, component.common_name.as_ref().unwrap_or(&component.unique_name));
				});
			}
		});
	})
}
			});
		});
	}
}