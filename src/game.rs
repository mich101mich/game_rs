use super::{
	log, ui,
	world::{GamePos, Machine, MachineType, TilePos, World},
	Backend, BackendStyle, Color,
};

pub struct Game {
	pub mouse: ui::Mouse,
	pub world: World,
	pub machines: Vec<Machine>,
}

impl Game {
	pub fn new() -> Self {
		log!("Starting...");
		let mut ret = Game {
			mouse: ui::Mouse::new(),
			world: World::new(64, 64),
			machines: vec![],
		};

		ret.add_machine(TilePos::new(32, 32), MachineType::Spawn);

		ret
	}

	pub fn draw(&mut self, backend: &mut Backend) {
		backend.fill(Color::rgb(128, 128, 128));
		self.world.draw(backend);

		for machine in &self.machines {
			machine.draw(backend);
		}

		backend.stroke_circle(GamePos::new(20.0, 100.0), 20.0, 5.0, Color::rgb(0, 255, 0));
	}

	pub fn end(&mut self) {}

	pub fn on_key_press(&mut self, code: Option<ui::KeyCode>, shift: bool, ctrl: bool) {
		self.mouse.set_shift(shift);
		self.mouse.set_ctrl(ctrl);
	}

	pub fn add_machine(&mut self, pos: TilePos, machine: MachineType) {
		self.machines
			.push(Machine::new(&mut self.world, pos, machine));
	}
}
