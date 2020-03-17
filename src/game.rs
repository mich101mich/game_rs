use super::{entity::*, world::*, *};

pub struct Game {
	pub mouse: ui::Mouse,
	pub world: World,
	pub entities: Entities,
	pub minerals: Vec<usize>,
	selection: Selection,
	update_interval: f32,
	update_carry: f32,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		let mut ret = Self {
			mouse: ui::Mouse::new(),
			world: World::new(64, 64),
			entities: Entities::new(),
			minerals: vec![0; Mineral::count()],
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

		ret.entities.add_worker((33, 33).into());

		// <temp>
		ret.minerals[Mineral::Crystal.num()] = 10;
		// </temp>

		ret
	}

	pub fn draw(&mut self, backend: &mut Backend, delta_time: f32) {
		self.update_carry += delta_time;
		if self.update_carry >= self.update_interval {
			self.world.update(self.get_mineral(Mineral::Crystal) > 0);
			self.entities.update(&mut self.world);
			self.update_carry = 0.0;
		}

		backend.fill(Color::rgb(128, 128, 128));
		self.world.draw(backend);

		self.entities.draw(backend);

		self.mouse.draw(backend);
	}

	pub fn end(&mut self) {}

	pub fn on_mouse_event(&mut self, event: ui::MouseEvent) {
		use ui::SelectionInfo::*;
		match self.mouse.on_event(event) {
			Click(pos) => {
				let w_pos: TilePos = pos.into();
				if let Some(worker) = self.entities.worker_at(w_pos) {
					self.selection = Selection::Workers(std::iter::once(worker.id).collect());
				} else if self.world.machine_at(w_pos).is_some() {
					self.selection = Selection::Machine(w_pos);
				} else {
					self.selection = Selection::Tile(w_pos);
				}
			}
			Brush(pos, radius) => {
				let mut selected = HashSet::default();
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

#[derive(Debug)]
enum Selection {
	Nothing,
	Workers(HashSet<WorkerID>),
	Walls(HashSet<TilePos>),
	Machine(TilePos),
	Tile(TilePos),
}
