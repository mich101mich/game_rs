use super::{BackendStyle, TEXT_SIZE};
use crate::{ui::KeyCode, world::Dir, Game};
use sfml::graphics::*;

pub use sfml::graphics::Color;

pub struct Backend<'a> {
	window: RenderWindow,
	font: Font,
	assets: Vec<Vec<Sprite<'a>>>,
}

impl<'a> BackendStyle for Backend<'a> {
	fn start(mut game: Game) {
		let image = Image::from_memory(include_bytes!("../../assets/assets.png"))
			.expect("Unable to load assets.png");
		let texture = Texture::from_image(&image).expect("Unable to load Assets");

		let mut backend = Backend {
			window: RenderWindow::new((640, 480), "game", Default::default(), &Default::default()),
			font: Font::from_memory(include_bytes!("../../assets/consola.ttf"))
				.expect("Unable to load Font"),
			assets: Vec::new(),
		};

		{
			let rows = image.size().y / 16;
			let cols = image.size().x / 16;

			for y in 0..rows {
				let mut row = Vec::with_capacity(cols as usize);
				for x in 0..cols {
					let mut sprite = Sprite::with_texture(&texture);
					sprite.set_texture_rect(&IntRect::new(x as i32 * 16, y as i32 * 16, 16, 16));
					row.push(sprite);
				}
				backend.assets.push(row);
			}
		}

		backend.window.set_framerate_limit(60);

		'game_loop: loop {
			// Process events
			while let Some(event) = backend.window.poll_event() {
				use sfml::window::Event::*;
				match event {
					Closed => {
						game.end();
						backend.window.close();
						break 'game_loop;
					}
					KeyPressed {
						code, ctrl, shift, ..
					} => {
						game.on_key_press(convert_key_code(code), shift, ctrl);
					}
					_ => {}
				}
			}

			game.draw(&mut backend);

			backend.window.display();
		}
	}

	fn get_width(&self) -> u32 {
		self.window.size().x
	}
	fn get_height(&self) -> u32 {
		self.window.size().y
	}

	fn fill(&mut self, color: Color) {
		self.window.clear(&color);
	}

	fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Color) {
		let line = [
			Vertex::with_pos_color(start, color),
			Vertex::with_pos_color(end, color),
		];
		self.window
			.draw_primitives(&line, PrimitiveType::Lines, Default::default());
	}

	fn fill_rect(&mut self, pos: (f32, f32), size: (f32, f32), color: Color) {
		let mut rect = RectangleShape::new();
		rect.set_position(pos);
		rect.set_size(size);
		rect.set_fill_color(&color);
		self.window.draw(&rect);
	}
	fn stroke_rect(
		&mut self,
		(x, y): (f32, f32),
		(width, height): (f32, f32),
		line_width: f32,
		color: Color,
	) {
		let o = line_width / 2.0;

		let mut rect = RectangleShape::new();
		rect.set_position((x + o, y + o));
		rect.set_size((width - 2.0 * o, height - 2.0 * o));
		rect.set_outline_color(&color);
		rect.set_outline_thickness(line_width);
		rect.set_fill_color(&Color::TRANSPARENT);
		self.window.draw(&rect);
	}

	fn fill_circle(&mut self, (x, y): (f32, f32), radius: f32, color: Color) {
		let mut circle = CircleShape::new(radius, 50);
		circle.set_position((x - radius, y - radius));
		circle.set_fill_color(&color);
		self.window.draw(&circle);
	}
	fn stroke_circle(&mut self, (x, y): (f32, f32), radius: f32, line_width: f32, color: Color) {
		let o = line_width / 2.0;

		let mut circle = CircleShape::new(radius - o, 50);
		circle.set_position((x - radius + o, y - radius + o));
		circle.set_outline_color(&color);
		circle.set_outline_thickness(line_width);
		circle.set_fill_color(&Color::TRANSPARENT);
		self.window.draw(&circle);
	}

	fn draw_text(&mut self, text: &str, pos: (f32, f32), color: Color) {
		let mut elem = Text::new(text, &self.font, TEXT_SIZE as u32);
		elem.set_position(pos);
		elem.set_fill_color(&color);
		self.window.draw(&elem);
	}

	fn draw_asset(&mut self, row: usize, id: usize, target_pos: (f32, f32)) {
		let sprite = &mut self.assets[row][id];
		sprite.set_position(target_pos);
		self.window.draw(sprite);
	}
}

#[macro_export]
macro_rules! log {
	( $( $x: expr ),* ) => {
		println!($( $x ),*)
	};
}

use sfml::window::Key;
fn convert_key_code(key: sfml::window::Key) -> Option<KeyCode> {
	use Key::*;

	match key {
		c if c >= A && c <= Z => Some(KeyCode::Letter((c as u8 - A as u8 + b'a') as char)),
		n if n >= Num0 && n <= Num9 => Some(KeyCode::Number(n as usize - Num0 as usize)),
		n if n >= Numpad0 && n <= Numpad9 => Some(KeyCode::Number(n as usize - Numpad0 as usize)),

		Space => Some(KeyCode::Space),
		Escape => Some(KeyCode::Escape),
		Return => Some(KeyCode::Return),
		BackSpace => Some(KeyCode::Backspace),
		Delete => Some(KeyCode::Delete),

		Left => Some(KeyCode::Arrow(Dir::LEFT)),
		Right => Some(KeyCode::Arrow(Dir::RIGHT)),
		Up => Some(KeyCode::Arrow(Dir::UP)),
		Down => Some(KeyCode::Arrow(Dir::DOWN)),

		_ => None,
	}
}
