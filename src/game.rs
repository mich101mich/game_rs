use super::{
	log, ui,
	world::{GamePos, MachineType, Material, Mineral, World},
	Backend, BackendStyle, Color,
};

pub struct Game {
	pub mouse: ui::Mouse,
	pub world: World,
	pub minerals: Vec<usize>,
	pub update_interval: f32,
	pub update_carry: f32,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		let mut ret = Game {
			mouse: ui::Mouse::new(),
			world: World::new(64, 64),
			minerals: std::iter::repeat(0).take(Mineral::count()).collect(),
			update_interval: 1.0,
			update_carry: 0.0,
		};

		ret.world.add_machine((32, 32), MachineType::Spawn);
		for i in 1..4 {
			ret.world.set((32 + i, 32), Material::Platform);
			ret.world.set((32 + 3, 32 + i), Material::Platform);
		}

		ret.world.add_machine((32 + 3, 32 + 3), MachineType::Lab);

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

		self.mouse.draw(backend);
	}

	pub fn end(&mut self) {}

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
