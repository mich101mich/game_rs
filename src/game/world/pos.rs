
#[derive(Clone, Copy, Debug, PartialEq, Default, Add, Sub)]
pub struct GamePos {
	x: f32,
	y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash, Add, Sub)]
pub struct TilePos {
	x: u32,
	y: u32,
}

impl GamePos {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl TilePos {
	pub fn new(x: u32, y: u32) -> Self {
		Self { x, y }
	}
}
