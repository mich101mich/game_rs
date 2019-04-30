#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Material {
	Air,
	Platform,
	Debris,

	Bedrock,
	Granite,
	Rock,
	Ore,
	Crystal,
	Machine,
}
impl Material {
	pub fn is_solid(self) -> bool {
		self as usize >= Material::Bedrock as usize
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

pub use Material::*;
