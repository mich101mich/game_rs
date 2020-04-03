mod button;
mod clickable;
mod key;
mod menu;
mod mouse;
pub use button::*;
pub use clickable::*;
pub use key::KeyCode;
pub use menu::*;
pub use mouse::*;

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
