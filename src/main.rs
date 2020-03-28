#![allow(clippy::new_without_default)]

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

mod backend;
pub use backend::{Backend, BackendStyle, Color};

mod game;
pub use game::*;

pub mod ui;

pub mod entity {
	mod entities;
	mod exec;
	mod item;
	mod job;
	mod scheduler;
	mod worker;
	pub use entities::*;
	pub use exec::*;
	pub use item::*;
	pub use job::*;
	pub use scheduler::*;
	pub use worker::*;
}

pub mod world {
	mod dir;
	mod grid;
	mod machine;
	mod material;
	mod pos;
	mod wrapper;
	pub use dir::*;
	pub use grid::*;
	pub use machine::*;
	pub use material::*;
	pub use pos::*;
	pub use wrapper::*;
}

pub use fnv::FnvHashMap as HashMap;
pub use fnv::FnvHashSet as HashSet;

#[cfg(target_arch = "wasm32")]
fn log_panic(info: &std::panic::PanicInfo) {
	let p = info.payload();
	if let Some(s) = p.downcast_ref::<&str>() {
		err!("panic occurred: {:?}", s);
	} else if let Some(s) = p.downcast_ref::<String>() {
		err!("panic occurred: {:?}", s);
	} else {
		err!("panic occurred");
	}
}

fn main() {
	#[cfg(target_arch = "wasm32")]
	std::panic::set_hook(Box::new(log_panic));

	let game = Game::new();

	Backend::start(game);
}

#[macro_export]
macro_rules! make_id {
	($name: ident, $display: tt) => {
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}({})", stringify!($display), self.0)
			}
		}
		impl From<$name> for usize {
			fn from(id: $name) -> usize {
				id.0
			}
		}
		impl From<usize> for $name {
			fn from(id: usize) -> $name {
				$name(id)
			}
		}
	};
}
