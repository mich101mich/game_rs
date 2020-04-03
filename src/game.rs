use super::{entity::*, ui::*, world::*, *};

static mut TIME: f32 = 0.0;

pub struct Game {
	pub mouse: Mouse,
	pub world: World,
	pub entities: Entities,
	pub scheduler: Scheduler,
	pub minerals: Vec<usize>,
	pub menu: Menu,
	pub update_interval: f32,
	pub update_carry: f32,
	pub tick: usize,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");

		let size = 128;
		let mid = size / 2;

		let mut ret = Self {
			mouse: Mouse::new(),
			world: World::new(size, size),
			entities: Entities::new(),
			scheduler: Scheduler::new(),
			minerals: vec![0; Mineral::count()],
			menu: Menu::new(),
			update_interval: 0.2,
			update_carry: 0.0,
			tick: 0,
		};

		ret.world.add_machine((mid, mid), MachineType::Spawn);
		for i in 1..4 {
			ret.world.set((mid + i, mid), Material::Platform);
			ret.world.set((mid + 3, mid + i), Material::Platform);
		}

		ret.world.add_machine((mid + 3, mid + 3), MachineType::Lab);

		ret.entities.add_worker((mid + 2, mid + 2).into());

		// TODO: <temp>
		ret.minerals[Mineral::Crystal.num()] = 10;
		ret.entities.add_item(
			((mid - 2) as f32 * 16.0 + 5.0, (mid - 2) as f32 * 16.0 + 5.0).into(),
			Mineral::Crystal,
		);
		ret.entities.add_item(
			((mid - 2) as f32 * 16.0, (mid - 2) as f32 * 16.0 + 5.0).into(),
			Mineral::Ore,
		);
		// </temp>

		ret
	}

	/// Returns the current Game-tick. The fractal part is the current frame's progress to the next tick
	pub fn time() -> f32 {
		unsafe { TIME }
	}

	pub fn resize(&mut self, backend: &Backend) {
		self.mouse.set_center(
			TilePos::new(self.world.width() / 2, self.world.height() / 2).into(),
			GamePos::new(backend.get_width() as f32, backend.get_height() as f32),
		);
		self.menu.set_pos(backend.get_width() as f32);
		self.world.set_dirty();
	}

	pub fn draw(&mut self, backend: &mut Backend, delta_time: f32) {
		self.update_carry += delta_time;
		if self.update_carry >= self.update_interval {
			self.tick += 1;
			if self.tick == std::usize::MAX {
				self.tick = 0;
			}
			self.world.update(self.get_mineral(Mineral::Crystal) > 0);
			self.scheduler.update(&mut self.entities, &mut self.world);
			self.update_carry = 0.0;
		}

		unsafe {
			TIME = self.tick as f32 + self.update_carry / self.update_interval;
		}

		backend.fill(Colors::Background);
		self.world.draw(backend);

		self.entities.draw(backend);

		self.menu.draw(backend, &self.entities);

		self.mouse.draw(backend);
	}

	pub fn end(&mut self) {}

	pub fn on_mouse_event(&mut self, event: MouseEvent) {
		use SelectionInfo::*;
		match self.mouse.on_event(event) {
			Click(pos) => {
				let w_pos: TilePos = pos.into();
				let selection = if self
					.menu
					.process_click(self.mouse.world_to_screen(pos), &mut self.entities)
				{
					return;
				} else if let Some(entity) = self.entities.entity_at(pos) {
					match entity {
						Entity::Item(id) => Selection::Item(id),
						Entity::Worker(id) => Selection::Workers(std::iter::once(id).collect()),
					}
				} else if self.world.machine_at(w_pos).is_some() {
					Selection::Machine(w_pos)
				} else if self.world.is_visible(w_pos) {
					// TODO: <temp>
					let mut w = self
						.entities
						.workers_mut()
						.next()
						.expect("You killed my Worker :/");
					w.next_target = self.world.path(w.pos, w_pos).map(|path| (w_pos, path));
					// </temp>
					Selection::Air(w_pos)
				} else {
					Selection::Nothing
				};
				self.menu.set_selection(selection, &self.entities);
			}
			Brush(pos, radius, append) => {
				let tl: TilePos = (pos - GamePos::new(radius, radius)).into();
				let br: TilePos = (pos + GamePos::new(radius, radius) + GamePos::TILE).into();
				let hitbox = Hitbox::Circle { pos, radius };

				let world = &self.world;
				let tiles = tl
					.rect_iter(br)
					.filter(|&tile| {
						hitbox.intersects(Hitbox::Rect {
							pos: tile.into(),
							size: GamePos::TILE,
						})
					})
					.filter(|&tile| world.is_visible(tile) && world.is_solid(tile));

				let mut selection = if append {
					match self.menu.take_selection() {
						Selection::Walls(sel) => sel,
						_ => HashSet::default(),
					}
				} else {
					HashSet::default()
				};

				for tile in tiles {
					selection.insert(tile);
				}

				self.menu
					.set_selection(Selection::Walls(selection), &self.entities);
			}
			Area(top_left, bottom_right) => {
				let hitbox = Hitbox::Rect {
					pos: top_left,
					size: bottom_right - top_left,
				};

				let selection = self
					.entities
					.workers()
					.filter(|w| hitbox.intersects(w.hitbox()))
					.map(|w| w.id)
					.collect();

				self.menu
					.set_selection(Selection::Workers(selection), &self.entities);
			}
			NoChange => (),
		}
	}

	pub fn on_key_press(&mut self, code: Option<KeyCode>, shift: ButtonState, ctrl: ButtonState) {
		self.mouse.set_shift(shift);
		self.mouse.set_ctrl(ctrl);

		if code == Some(KeyCode::Letter('h')) {
			self.world.toggle_debug_mode();
		}
	}

	pub fn get_mineral(&self, mineral: Mineral) -> usize {
		self.minerals[mineral.num()]
	}
}
