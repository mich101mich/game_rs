use crate::world::Dir;

#[derive(Debug, PartialEq, Eq)]
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
