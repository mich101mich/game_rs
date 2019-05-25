#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Material {
	Air,
	Bedrock,
	Granite,
	Rock,
	Ore,
	Crystal,
	Debris,
	Platform,
	Machine,
}

use Material::*;

impl Material {
	pub fn is_solid(self) -> bool {
		const NOT_SOLID: u32 = (1 << Air as u32) | (1 << Platform as u32) | (1 << Debris as u32);
		(NOT_SOLID & (1 << self as u32)) == 0
	}
	pub fn walk_cost(self) -> Option<usize> {
		match self {
			Air => Some(2),
			Platform => Some(1),
			Debris => Some(4),
			_ => None, // solid
		}
	}
}

pub enum Mineral {
	Ore,
	Crystal,
}

impl Mineral {
	pub fn count() -> usize {
		2
	}
	pub fn num(self) -> usize {
		self as usize
	}
}
