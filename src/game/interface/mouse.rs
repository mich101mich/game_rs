use crate::game::GamePos;

#[derive(Default, Debug)]
pub struct Mouse {
	pos: GamePos,
	start_pos: Option<GamePos>,
	shift: bool,
	ctrl: bool,
}

impl Mouse {
	pub fn new() -> Mouse {
		Default::default()
	}
	pub fn set_shift(&mut self, shift: bool) {
		self.shift = shift;
	}
	pub fn set_ctrl(&mut self, ctrl: bool) {
		self.ctrl = ctrl;
	}
}
