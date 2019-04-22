
use crate::game::{Dir};

pub enum KeyCode {
	Number(usize),
	Letter(char),
	Space,
	Escape,
	Return,
	Backspace,
	Delete,
	Arrow(Dir),

}
