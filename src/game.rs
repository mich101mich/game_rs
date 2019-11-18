use super::{log, ui, world::*, Backend, BackendStyle, Color};
use std::collections::HashSet;

pub struct Game {
	pub mouse: ui::Mouse,
	pub world: World,
	pub scheduler: Scheduler,
	pub minerals: Vec<usize>,
	selection: Selection,
	update_interval: f32,
	update_carry: f32,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		let mut ret = Game {
			mouse: ui::Mouse::new(),
			world: World::new(64, 64),
			scheduler: Scheduler::new(),
			minerals: std::iter::repeat(0).take(Mineral::count()).collect(),
			selection: Selection::Nothing,
			update_interval: 1.0,
			update_carry: 0.0,
		};

		ret.world.add_machine((32, 32), MachineType::Spawn);
		for i in 1..4 {
			ret.world.set((32 + i, 32), Material::Platform);
			ret.world.set((32 + 3, 32 + i), Material::Platform);
		}

		ret.world.add_machine((32 + 3, 32 + 3), MachineType::Lab);

		ret.scheduler.add_worker((33, 33).into());

		ret
	}

	pub fn draw(&mut self, backend: &mut Backend, delta_time: f32) {
		self.update_carry += delta_time;
		if self.update_carry >= self.update_interval {
			self.world.update(self.get_mineral(Mineral::Crystal) > 0);
			self.update_carry = 0.0;
		}

		backend.fill(Color::rgb(128, 128, 128));
		self.world.draw(backend);

		self.scheduler.draw(backend);

		self.mouse.draw(backend);
	}

	pub fn end(&mut self) {}

	pub fn on_mouse_event(&mut self, event: ui::MouseEvent) {
		use ui::SelectionInfo::*;
		match self.mouse.on_event(event) {
			Click(pos) => {
				// TODO: search for machine, worker, tile
			}
			Brush(pos, radius) => {
				let mut selected = HashSet::new();
				let left = (pos.x - radius).floor() as usize / TILE_SIZE;
				let top = (pos.y - radius).floor() as usize / TILE_SIZE;
				let right = (pos.x + radius).ceil() as usize / TILE_SIZE;
				let bottom = (pos.y + radius).ceil() as usize / TILE_SIZE;

				for y in top..bottom {
					for x in left..right {
						// check drillable
						if self.world.is_solid((x, y)) {
							selected.insert((x, y).into());
						}
					}
				}

				self.selection = Selection::Walls(selected);
			}
			AppendBrush(pos, radius) => {
				// TODO: copy code from above
			}
			Area(top_left, bottom_right) => {
				// TODO: find workers
			}
			NoChange => (),
		}
	}

	pub fn on_key_press(
		&mut self,
		code: Option<ui::KeyCode>,
		shift: ui::ButtonState,
		ctrl: ui::ButtonState,
	) {
		self.mouse.set_shift(shift);
		self.mouse.set_ctrl(ctrl);
	}

	pub fn get_mineral(&self, mineral: Mineral) -> usize {
		self.minerals[mineral.num()]
	}
}

enum Selection {
	Nothing,
	Workers(HashSet<usize>),
	Walls(HashSet<TilePos>),
	Machine(TilePos),
}
