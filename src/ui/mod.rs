mod mouse;
pub use mouse::*;

mod key;
pub use key::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
	Up,
	Down,
}

impl Default for ButtonState {
	fn default() -> Self {
		Self::Up
	}
}

impl Into<bool> for ButtonState {
	fn into(self) -> bool {
		match self {
			ButtonState::Up => false,
			ButtonState::Down => true,
		}
	}
}
impl Into<ButtonState> for bool {
	fn into(self) -> ButtonState {
		if self {
			ButtonState::Down
		} else {
			ButtonState::Up
		}
	}
}
