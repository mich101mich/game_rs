use super::{Dir, TilePos, World, TILE_SIZE};
use crate::HashSet;

#[derive(Debug, PartialEq, Eq)]
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
	power_source: Option<Dir>,
	power: bool,
	cooldown: Option<usize>,
}

impl Machine {
	pub fn new(pos: TilePos, machine_type: MachineType) -> Self {
		Self {
			pos,
			machine_type,
			power_source: None,
			power: false,
			cooldown: None,
		}
	}

	pub fn draw(&self, backend: &mut crate::Backend) {
		use crate::{BackendStyle, Colors};
		if self.machine_type != MachineType::Platform {
			backend.draw_asset((1, self.machine_type.num()), self.pos);
		}
		if !self.power {
			backend.fill_rect(
				self.pos,
				(TILE_SIZE as f32, TILE_SIZE as f32),
				Colors::NoPower,
			);
			backend.draw_asset((3, 0), self.pos);
		}
	}

	pub fn update(&mut self, spawn_has_power: bool) {
		self.power = matches!(self.machine_type, ConstructionSite(..))
			|| (self.has_power_source() && spawn_has_power);
	}

	pub fn remove(&mut self) {}

	pub fn is_spawn(&self) -> bool {
		self.machine_type == MachineType::Spawn
	}

	pub fn has_power_source(&self) -> bool {
		self.is_spawn() || self.power_source.is_some()
	}
	pub fn set_power_source(&mut self, source: Option<Dir>) {
		self.power_source = source;
	}
	pub fn find_power_source(&self, world: &World) -> Option<Dir> {
		Dir::all().find_map(|dir| {
			let mut pos = world.tile_in_dir(self.pos, dir)?;
			let mut seen = HashSet::default();
			while seen.insert(pos) {
				let machine = world.machine_at(pos)?;
				if machine.is_spawn() {
					return Some(dir);
				}
				pos = world.tile_in_dir(machine.pos, machine.power_source?)?;
			}
			None
		})
	}

	#[allow(clippy::option_option)]
	pub fn power_source_changed(&self, world: &World) -> Option<Option<Dir>> {
		if self.is_spawn() {
			return None;
		}
		let mut source = self.power_source;
		if !source
			.and_then(|dir| world.tile_in_dir(self.pos, dir))
			.and_then(|p| world.machine_at(p))
			.map(|m| m.has_power_source())
			.unwrap_or(false)
		{
			source = None
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
