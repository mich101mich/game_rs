use super::{entity::*, ui::*, world::*, *};

pub struct Game {
	pub mouse: Mouse,
	pub world: World,
	pub entities: Entities,
	pub minerals: Vec<usize>,
	pub menu: Menu,
	pub update_interval: f32,
	pub update_carry: f32,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		let mut ret = Self {
			mouse: Mouse::new(),
			world: World::new(64, 64),
			entities: Entities::new(),
			minerals: vec![0; Mineral::count()],
			menu: Menu::new(),
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

		// TODO: <temp>
		ret.minerals[Mineral::Crystal.num()] = 10;
		ret.entities
			.add_item((490.0, 490.0).into(), Mineral::Crystal);
		ret.entities.add_item((480.0, 490.0).into(), Mineral::Ore);
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

		self.menu.draw(backend, &self.entities);

		self.mouse.draw(backend);
	}

	pub fn end(&mut self) {}

	pub fn on_mouse_event(&mut self, event: MouseEvent) {
		use SelectionInfo::*;
		match self.mouse.on_event(event) {
			Click(pos) => {
				let w_pos: TilePos = pos.into();
				self.menu.selection = if let Some(entity) = self.entities.entity_at(pos) {
					match entity {
						Clicked::Item(id) => Selection::Item(id),
						Clicked::Worker(id) => Selection::Workers(std::iter::once(id).collect()),
					}
				} else if self.world.machine_at(w_pos).is_some() {
					Selection::Machine(w_pos)
				} else if self.world.is_visible(w_pos) {
					Selection::Ground(w_pos)
				} else {
					Selection::Nothing
				};
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

				if !append {
					self.menu.selection = Selection::Walls(HashSet::default());
				}

				let selection = match &mut self.menu.selection {
					Selection::Walls(sel) => sel,
					s => {
						*s = Selection::Walls(HashSet::default());
						match s {
							Selection::Walls(sel) => sel,
							_ => unreachable!(),
						}
					}
				};

				for tile in tiles {
					selection.insert(tile);
				}
			}
			Area(top_left, bottom_right) => {
				let hitbox = Hitbox::Rect {
					pos: top_left,
					size: bottom_right - top_left,
				};
				crate::log!("{:?}", hitbox);
				crate::log!("{:?}", self.entities.workers().next().unwrap().hitbox());
				crate::log!("{:?}", self.entities.workers().next().unwrap().hitbox().intersects(hitbox));

				let selection = self
					.entities
					.workers()
					.filter(|w| hitbox.intersects(w.hitbox()))
					.map(|w| w.id)
					.collect();

				self.menu.selection = Selection::Workers(selection);
			}
			NoChange => (),
		}
	}

	pub fn on_key_press(&mut self, code: Option<KeyCode>, shift: ButtonState, ctrl: ButtonState) {
		self.mouse.set_shift(shift);
		self.mouse.set_ctrl(ctrl);
	}

	pub fn get_mineral(&self, mineral: Mineral) -> usize {
		self.minerals[mineral.num()]
	}
}
