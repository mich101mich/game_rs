use super::ButtonState;
use crate::{world::GamePos};

#[derive(Debug, Default)]
pub struct Mouse {
	pos: GamePos,
	start_pos: Option<GamePos>,
	left: ButtonState,
	right: ButtonState,
	shift: ButtonState,
	ctrl: ButtonState,
	scale: f32,
	offset: GamePos,
}

impl Mouse {
	pub fn new() -> Mouse {
		Mouse {
			scale: 1.0,
			..Default::default()
		}
	}

	pub fn on_event(&mut self, event: MouseEvent) {
		use MouseEvent::*;
		match event {
			Move(delta) => {
				if self.left_down() {
					let moved = delta / self.scale;
					self.offset += moved;
				}
				self.pos += delta;
			}
			ClickDown(button) => {
				self.set_button(button, ButtonState::Down);
			}
			ClickUp(button) => {
				self.set_button(button, ButtonState::Up);
			}
			Scroll(delta) => {
				let factor = 1.0 - delta / 10.0;
				self.scale *= factor;

				self.offset -= self.pos / (self.scale / factor) - self.pos / self.scale;
			}
		}
	}

	pub fn set_shift(&mut self, shift: ButtonState) {
		self.shift = shift;
	}
	pub fn set_ctrl(&mut self, ctrl: ButtonState) {
		self.ctrl = ctrl;
	}
	pub fn set_button(&mut self, button: MouseButton, state: ButtonState) {
		match button {
			MouseButton::Left => {
				self.left = state;
			}
			MouseButton::Right => {
				self.right = state;
			}
		}
	}
	pub fn left(&self) -> ButtonState {
		self.left
	}
	pub fn left_down(&self) -> bool {
		self.left == ButtonState::Down
	}
	pub fn right(&self) -> ButtonState {
		self.right
	}
	pub fn right_down(&self) -> bool {
		self.right == ButtonState::Down
	}
	pub fn shift(&self) -> ButtonState {
		self.shift
	}
	pub fn shift_down(&self) -> bool {
		self.shift == ButtonState::Down
	}
	pub fn ctrl(&self) -> ButtonState {
		self.ctrl
	}
	pub fn ctrl_down(&self) -> bool {
		self.ctrl == ButtonState::Down
	}

	pub fn scale(&self) -> f32 {
		self.scale
	}
	pub fn offset(&self) -> GamePos {
		self.offset
	}
}

#[derive(Debug)]
pub enum MouseButton {
	Left,
	Right,
}

#[derive(Debug)]
pub enum MouseEvent {
	Move(GamePos),
	ClickDown(MouseButton),
	ClickUp(MouseButton),
	Scroll(f32),
}
