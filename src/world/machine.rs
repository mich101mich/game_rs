use super::{Material, TilePos, World};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MachineType {
	Spawn,
	Lab,
	ConstructionSite(Box<MachineType>),
}

impl MachineType {
	pub fn num(&self) -> usize {
		use MachineType::*;
		match *self {
			Spawn => 0,
			Lab => 1,
			ConstructionSite(..) => 2,
		}
	}
}

#[derive(Debug)]
pub struct Machine {
	pub pos: TilePos,
	pub machine_type: MachineType,
	has_power: bool,
	cooldown: Option<usize>,
}

impl Machine {
	pub fn new(world: &mut World, pos: TilePos, machine_type: MachineType) -> Machine {
		world.set_p(pos, Material::Machine);

		Machine {
			pos,
			machine_type,
			has_power: false,
			cooldown: None,
		}
	}

	pub fn draw(&self, backend: &mut crate::Backend) {
		use crate::{BackendStyle, Color};
		backend.draw_asset((1, self.machine_type.num()), self.pos.into());
		if !self.has_power {
			backend.fill_rect(
				self.pos.into(),
				(16.0, 16.0).into(),
				Color::rgba(128, 128, 128, 100),
			);
		}
	}
}
