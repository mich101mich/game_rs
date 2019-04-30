use crate::world::GamePos;

#[derive(Debug)]
pub struct Mouse {
	pos: GamePos,
	start_pos: Option<GamePos>,
	shift: bool,
	ctrl: bool,
}

impl Mouse {
	pub fn new() -> Mouse {
		Mouse {
			pos: GamePos::new(0.0, 0.0),
			start_pos: None,
			shift: false,
			ctrl: false,
		}
	}
	pub fn set_shift(&mut self, shift: bool) {
		self.shift = shift;
	}
	pub fn set_ctrl(&mut self, ctrl: bool) {
		self.ctrl = ctrl;
	}
}
