use super::{BackendStyle, TEXT_SIZE};
use crate::{
	ui::KeyCode,
	world::{Dir, GamePos},
	Game,
};
use sfml::{
	graphics::*,
	window::{self, VideoMode},
};

pub use sfml::graphics::Color;

pub struct Backend<'a> {
	window: RenderWindow,
	font: Font,
	assets: Vec<Vec<Sprite<'a>>>,
	background: RenderTexture,
}

impl<'a> BackendStyle for Backend<'a> {
	fn start(mut game: Game) {
		let image = Image::from_memory(include_bytes!("../../assets/assets.png"))
			.expect("Unable to load assets.png");
		let texture = Texture::from_image(&image).expect("Unable to load Assets");

		let background = RenderTexture::new(
			game.world.width() as u32 * 16,
			game.world.height() as u32 * 16,
			false,
		)
		.unwrap();

		let mut video_mode: VideoMode = (640, 480).into();
		let mut style: window::Style = Default::default();
		if let Some(mode) = VideoMode::fullscreen_modes().first() {
			video_mode = *mode;
			style = window::Style::FULLSCREEN;
		}

		let mut backend = Backend {
			window: RenderWindow::new(video_mode, "game", style, &Default::default()),
			font: Font::from_memory(include_bytes!("../../assets/consola.ttf"))
				.expect("Unable to load Font"),
			assets: Vec::new(),
			background,
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

		let mut clock = sfml::system::Clock::start();

		'game_loop: loop {
			// Process events
			while let Some(event) = backend.window.poll_event() {
				use sfml::window::Event::*;
				match event {
					Closed => {
						break 'game_loop;
					}
					KeyPressed {
						code, ctrl, shift, ..
					} => {
						if shift && code == window::Key::Escape {
							break 'game_loop;
						}
						game.on_key_press(convert_key_code(code), shift, ctrl);
					}
					Resized { width, height } => backend.window.set_view(&View::from_rect(
						&FloatRect::new(0.0, 0.0, width as f32, height as f32),
					)),
					_ => {}
				}
			}

			game.draw(&mut backend, clock.restart().as_seconds());

			backend.window.display();
		}

		game.end();
		backend.window.close();
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

	fn draw_line<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, start: T, end: T2, color: Color) {
		let line = [
			Vertex::with_pos_color(start.into(), color),
			Vertex::with_pos_color(end.into(), color),
		];
		self.window
			.draw_primitives(&line, PrimitiveType::Lines, Default::default());
	}

	fn fill_rect<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, pos: T, size: T2, color: Color) {
		let mut rect = RectangleShape::new();
		rect.set_position(pos.into());
		rect.set_size(size.into());
		rect.set_fill_color(&color);
		self.window.draw(&rect);
	}
	fn stroke_rect<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, pos: T, size: T2, line_width: f32, color: Color) {
		let o = GamePos::new(line_width, line_width) / 2.0;

		let mut rect = RectangleShape::new();
		rect.set_position(pos.into() + o);
		rect.set_size(size.into() + 2.0 * o);
		rect.set_outline_color(&color);
		rect.set_outline_thickness(line_width);
		rect.set_fill_color(&Color::TRANSPARENT);
		self.window.draw(&rect);
	}

	fn fill_circle<T: Into<GamePos>>(&mut self, pos: T, radius: f32, color: Color) {
		let GamePos {x, y} = pos.into();

		let mut circle = CircleShape::new(radius, 50);
		circle.set_position((x - radius, y - radius));
		circle.set_fill_color(&color);
		self.window.draw(&circle);
	}
	fn stroke_circle<T: Into<GamePos>>(&mut self, pos: T, radius: f32, line_width: f32, color: Color) {
		let GamePos {x, y} = pos.into();
		let o = line_width / 2.0;

		let mut circle = CircleShape::new(radius - o, 50);
		circle.set_position((x - radius + o, y - radius + o));
		circle.set_outline_color(&color);
		circle.set_outline_thickness(line_width);
		circle.set_fill_color(&Color::TRANSPARENT);
		self.window.draw(&circle);
	}

	fn draw_text<T: Into<GamePos>>(&mut self, text: &str, pos: T, color: Color) {
		let mut elem = Text::new(text, &self.font, TEXT_SIZE as u32);
		elem.set_position(pos.into());
		elem.set_fill_color(&color);
		self.window.draw(&elem);
	}

	fn draw_asset<T: Into<GamePos>>(&mut self, (row, id): (usize, usize), target_pos: T) {
		let sprite = &mut self.assets[row][id];
		sprite.set_position(target_pos.into());
		self.window.draw(sprite);
	}

	fn draw_background(&mut self) {
		self.background.display();
		let sprite = Sprite::with_texture(self.background.texture());
		self.window.draw(&sprite);
	}

	fn clear_background(&mut self) {
		self.background.clear(&Color::BLACK);
	}

	fn draw_to_background<T: Into<GamePos>>(&mut self, (row, id): (usize, usize), target_pos: T) {
		let sprite = &mut self.assets[row][id];
		sprite.set_position(target_pos.into());
		self.background.draw(sprite);
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

use sfml::system::Vector2f;
impl Into<Vector2f> for GamePos {
	fn into(self) -> Vector2f {
		Vector2f::new(self.x, self.y)
	}
}
