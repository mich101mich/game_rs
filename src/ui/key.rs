
use crate::world::Dir;

pub enum KeyCode {
	Number(usize),
	Letter(char),
	Space,
	Escape,
	Enter,
	Backspace,
	Delete,
	Arrow(Dir),
}
