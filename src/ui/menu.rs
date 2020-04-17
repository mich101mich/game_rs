use crate::{
	backend::TEXT_SIZE,
	entity::{Entities, ItemID, WorkerID},
	ui::{Button, Clickable},
	world::{GamePos, TilePos, World},
	Backend, BackendStyle, Colors, HashSet,
};

#[derive(Debug)]
pub struct Menu {
	selection: Selection,
	context_menu: Vec<Button<usize>>,
	pos: f32,
	width: f32,
}

impl Menu {
	pub fn new() -> Self {
		Self {
			selection: Selection::Nothing,
			context_menu: vec![],
			pos: 0.0,
			width: 0.0,
		}
	}

	pub fn set_pos(&mut self, width: f32) {
		self.width = (width / 10.0).min(200.0);
		self.pos = width - self.width - 5.0;

		for (i, button) in self.context_menu.iter_mut().enumerate() {
			button.pos = GamePos::new(self.pos, 5.0 + i as f32 * (TEXT_SIZE as f32 + 3.0));
			button.size = GamePos::new(self.width, TEXT_SIZE as f32 + 2.0);
		}
	}

	pub fn draw(&self, backend: &mut Backend, entities: &Entities) {
		match &self.selection {
			Nothing => {}
			Workers(workers) => {
				for id in workers.iter() {
					backend.stroke_hitbox(entities.worker(*id).hitbox(), 1.0, Colors::Highlight);
				}
			}
			Item(id) => {
				backend.stroke_hitbox(entities.item(*id).hitbox(), 1.0, Colors::Highlight);
			}
			Walls(tiles) => {
				for pos in tiles.iter() {
					backend.stroke_rect(*pos, GamePos::TILE, 1.0, Colors::Highlight);
				}
			}
			Machine(pos) => backend.stroke_rect(*pos, GamePos::TILE, 1.0, Colors::Highlight),
			Air(pos) => backend.stroke_rect(*pos, GamePos::TILE, 1.0, Colors::Highlight),
		}

		for button in self.context_menu.iter() {
			button.draw(backend);
		}
	}

	pub fn set_selection(&mut self, selection: Selection, entities: &Entities, world: &World) {
		self.selection = selection;
		self.context_menu = match &self.selection {
			Nothing => Box::new(std::iter::empty()),
			Workers(workers) => workers
				.iter()
				.next()
				.map(|id| entities.worker(*id).context_menu())
				.unwrap_or_else(|| Box::new(std::iter::empty())),
			Item(id) => entities.item(*id).context_menu(),
			Walls(tiles) => world.context_menu_walls(tiles),
			Machine(pos) | Air(pos) => world.context_menu_tile(*pos),
		}
		.enumerate()
		.map(|(i, &(identifier, text))| {
			Button::new(
				identifier,
				String::from(text),
				GamePos::new(self.pos, 5.0 + i as f32 * (TEXT_SIZE as f32 + 3.0)),
				GamePos::new(self.width, TEXT_SIZE as f32 + 2.0),
			)
		})
		.collect();
	}

	pub fn process_click(
		&mut self,
		pos: GamePos,
		entities: &mut Entities,
		world: &mut World,
	) -> bool {
		for button in self.context_menu.iter() {
			if button.contains(pos) {
				let clear = match &self.selection {
					Nothing => panic!("Why was there a Button when nothing is selected??"),
					Workers(workers) => {
						let mut clear = false;
						for id in workers {
							clear = entities
								.worker_mut(*id)
								.on_context_clicked(button.identifier)
								|| clear;
						}
						clear
					}
					Item(id) => entities.item_mut(*id).on_context_clicked(button.identifier),
					Walls(tiles) => world.context_click_walls(tiles, button.identifier),
					Machine(pos) | Air(pos) => world.context_click_tile(*pos, button.identifier),
				};
				if clear {
					self.set_selection(Selection::Nothing, entities, world);
				} else {
					let sel = self.take_selection();
					self.set_selection(sel, entities, world);
				}
				return true;
			}
		}
		false
	}
	pub fn take_selection(&mut self) -> Selection {
		std::mem::replace(&mut self.selection, Selection::Nothing)
	}
}

#[derive(Debug)]
pub enum Selection {
	Nothing,
	Workers(HashSet<WorkerID>),
	Item(ItemID),
	Walls(HashSet<TilePos>),
	Machine(TilePos),
	Air(TilePos),
}
use Selection::*;
