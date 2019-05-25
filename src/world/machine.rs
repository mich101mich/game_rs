use super::{Dir, TilePos, World};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MachineType {
	Spawn,
	Lab,
	ConstructionSite(Box<MachineType>),
	Platform,
}
use MachineType::*;

impl MachineType {
	pub fn num(&self) -> usize {
		match *self {
			Spawn => 0,
			Lab => 1,
			ConstructionSite(..) => 2,
			Platform => panic!("Platform has no number"),
		}
	}
}

#[derive(Debug)]
pub struct Machine {
	pub pos: TilePos,
	pub machine_type: MachineType,
	power_source: Option<(TilePos, Dir)>,
	power: bool,
	cooldown: Option<usize>,
}

impl Machine {
	pub fn new(pos: TilePos, machine_type: MachineType) -> Machine {
		Machine {
			pos,
			machine_type,
			power_source: None,
			power: false,
			cooldown: None,
		}
	}

	pub fn draw(&self, backend: &mut crate::Backend) {
		use crate::{BackendStyle, Color};
		if self.machine_type != MachineType::Platform {
			backend.draw_asset((1, self.machine_type.num()), self.pos.into());
		}
		if !self.power {
			backend.fill_rect(
				self.pos.into(),
				(16.0, 16.0).into(),
				Color::rgba(0, 0, 0, 140),
			);
			backend.draw_asset((3, 0), self.pos.into());
		}
	}

	pub fn update(&mut self, spawn_has_power: bool) {
		self.power = self.has_power_source() && spawn_has_power;
	}

	pub fn remove(&mut self) {}

	pub fn is_spawn(&self) -> bool {
		self.machine_type == MachineType::Spawn
	}

	pub fn has_power_source(&self) -> bool {
		self.is_spawn() || self.power_source.is_some()
	}
	pub fn get_power_source(&self) -> Option<TilePos> {
		if self.is_spawn() {
			Some(self.pos)
		} else {
			self.power_source.map(|(pos, _)| pos)
		}
	}
	pub fn set_power_source(&mut self, source: Option<(TilePos, Dir)>) {
		self.power_source = source;
	}
	pub fn find_power_source(&self, world: &World) -> Option<(TilePos, Dir)> {
		let mut sources = Dir::all().filter_map(|dir| {
			world
				.tile_in_dir(self.pos, dir)
				.and_then(|tile| world.machine(tile))
				.and_then(Machine::get_power_source)
				.map(|spawn| (spawn, dir))
		});
		sources.next()
	}

	#[allow(clippy::option_option)]
	pub fn power_source_changed(&self, world: &World) -> Option<Option<(TilePos, Dir)>> {
		if self.is_spawn() {
			return None;
		}
		let mut source = self.power_source;
		if let Some((_, spawn_dir)) = source {
			source = world
				.tile_in_dir(self.pos, spawn_dir)
				.and_then(|p| world.machine(p))
				.and_then(Machine::get_power_source)
				.map(|spawn_pos| (spawn_pos, spawn_dir));
		}
		if source.is_none() {
			source = self.find_power_source(world);
		}
		if source != self.power_source {
			Some(source)
		} else {
			None
		}
	}
}
