use super::{BackendStyle, Colors, TEXT_SIZE};
use crate::{
	ui,
	world::{GamePos, TILE_SIZE},
	Game,
};

use stdweb::{traits::*, unstable::TryInto, web::html_element::*, web::*};

static mut GAME: Option<Game> = None;
static mut BACKEND: Option<Backend> = None;
static mut TIME: f64 = 0.0;

#[macro_export]
macro_rules! log {
	( $( $x: expr ),* ) => {
		let s = format!($( $x ),*);
		js!(
			console.log(@{ s });
		);
	};
}

#[macro_export]
macro_rules! err {
	( $( $x: expr ),* ) => {
		let s = format!($( $x ),*);
		js!(
			console.error(@{ s });
		);
	};
}

fn game() -> &'static mut Game {
	unsafe { GAME.as_mut().unwrap() }
}

fn backend() -> &'static mut Backend {
	unsafe { BACKEND.as_mut().unwrap() }
}

fn update(time: f64) {
	window().request_animation_frame(update);

	let delta = time - unsafe { TIME };
	unsafe { TIME = time };

	let backend = backend();
	let mouse = &game().mouse;
	backend.ctx.set_transform(
		mouse.scale() as f64,
		0.0 as f64,
		0.0,
		mouse.scale() as f64,
		(mouse.offset().x * mouse.scale()) as f64,
		(mouse.offset().y * mouse.scale()) as f64,
	);
	game().draw(backend, delta as f32 / 1000.0);
}

fn on_mouse_wheel(event: event::MouseWheelEvent) {
	use stdweb::web::event::MouseWheelDeltaMode::*;
	let delta = match event.delta_mode() {
		Pixel => event.delta_y() / 100.0,
		Line => event.delta_y() / 3.0,
		Page => event.delta_y(),
	};
	game().on_mouse_event(ui::MouseEvent::Scroll(delta as f32));
}

fn on_mouse_move(event: event::MouseMoveEvent) {
	let mouse = &mut backend().mouse;
	let x = event.client_x();
	let y = event.client_y();
	let delta = ((x - mouse.0) as f32, (y - mouse.1) as f32);
	*mouse = (x, y);
	game().on_mouse_event(ui::MouseEvent::Move(delta.into()));
}

fn on_mouse_down(event: event::MouseDownEvent) {
	if event.button() == event::MouseButton::Left {
		game().on_mouse_event(ui::MouseEvent::ClickDown(ui::MouseButton::Left));
	} else if event.button() == event::MouseButton::Right {
		game().on_mouse_event(ui::MouseEvent::ClickDown(ui::MouseButton::Right));
	}
}
fn on_mouse_up(event: event::MouseUpEvent) {
	if event.button() == event::MouseButton::Left {
		game().on_mouse_event(ui::MouseEvent::ClickUp(ui::MouseButton::Left));
	} else if event.button() == event::MouseButton::Right {
		game().on_mouse_event(ui::MouseEvent::ClickUp(ui::MouseButton::Right));
	}
}

fn on_context_menu(event: event::ContextMenuEvent) {
	event.cancel_bubble();
	event.prevent_default();
}

fn on_key_down(event: event::KeyDownEvent) {
	game().on_key_press(
		convert_key_code(&event.key()),
		event.shift_key().into(),
		event.ctrl_key().into(),
	);
}

fn on_key_up(event: event::KeyUpEvent) {
	game().on_key_press(None, event.shift_key().into(), event.ctrl_key().into());
}

fn on_resize(_: event::ResizeEvent) {
	resize();
}
fn resize() {
	let backend = backend();

	let width: f64 = js! {
		return @{ backend.canvas.as_ref() }.getBoundingClientRect().width
	}
	.try_into()
	.unwrap();

	let height: f64 = js! {
		return @{ backend.canvas.as_ref() }.getBoundingClientRect().height
	}
	.try_into()
	.unwrap();

	let width = width as u32;
	let height = height as u32;

	backend.width = width;
	backend.height = height;
	backend.canvas.set_width(width);
	backend.canvas.set_height(height);

	backend
		.ctx
		.set_font(&format!("{}px consolas", TEXT_SIZE as f32 + 0.3));
	backend.ctx.set_text_baseline(TextBaseline::Top);
	js! {
		@{ &backend.ctx }.imageSmoothingEnabled = false;
		@{ &backend.bg }.imageSmoothingEnabled = false;
	}

	game().resize(&backend);
}

pub struct Backend {
	canvas: CanvasElement,
	background_canvas: CanvasElement,
	ctx: CanvasRenderingContext2d,
	bg: CanvasRenderingContext2d,
	assets: ImageElement,
	width: u32,
	height: u32,
	mouse: (i32, i32),
}
impl Backend {
	fn set_fill_style(&self, color: Colors) {
		js! { @{ &self.ctx }.fillStyle = COLOR[@{ color.num() }] }
	}
	fn set_stroke_style(&self, color: Colors) {
		js! { @{ &self.ctx }.strokeStyle = COLOR[@{ color.num() }] }
	}
}
impl BackendStyle for Backend {
	fn start(game: Game) {
		js! {
			window.COLOR = [
				@{ Color::from(Colors::Background).to_css() },
				@{ Color::from(Colors::Crystal).to_css() },
				@{ Color::from(Colors::Ore).to_css() },
				@{ Color::from(Colors::Worker).to_css() },
				@{ Color::from(Colors::NoPower).to_css() },
				@{ Color::from(Colors::Chunk).to_css() },
				@{ Color::from(Colors::Node).to_css() },
				@{ Color::from(Colors::Highlight).to_css() },
				@{ Color::from(Colors::Cursor).to_css() },
				@{ Color::from(Colors::Button).to_css() },
				@{ Color::from(Colors::Black).to_css() },
			];
		}

		document()
			.body()
			.unwrap()
			.set_attribute(
				"style",
				"margin: 0;  width: 100vw;  height: 99.9vh; background-color: black;",
			)
			.unwrap();

		let canvas: CanvasElement = document()
			.create_element("canvas")
			.unwrap()
			.try_into()
			.unwrap();

		canvas
			.set_attribute("style", "width: 100%; height: calc(100% - 3px);")
			.unwrap();

		document().body().unwrap().append_child(&canvas);

		let background_canvas: CanvasElement = document()
			.create_element("canvas")
			.unwrap()
			.try_into()
			.unwrap();

		background_canvas.set_width((game.world.width() * TILE_SIZE) as u32);
		background_canvas.set_height((game.world.height() * TILE_SIZE) as u32);

		let assets: &[u8] = include_bytes!("../../assets/assets.png");
		let assets = base64::encode(assets);

		let img: ImageElement = document()
			.create_element("img")
			.unwrap()
			.try_into()
			.unwrap();
		img.set_src(&(String::from("data:image/png;base64,") + &assets));
		img.set_attribute("hidden", "true").unwrap();
		document().body().unwrap().append_child(&img);

		unsafe {
			BACKEND = Some(Backend {
				ctx: canvas.get_context().unwrap(),
				bg: background_canvas.get_context().unwrap(),
				canvas,
				background_canvas,
				assets: img,
				width: 200,
				height: 200,
				mouse: (0, 0),
			});
			GAME = Some(game);
		}

		window().add_event_listener(on_resize);
		window().add_event_listener(on_mouse_up);
		window().add_event_listener(on_mouse_down);
		window().add_event_listener(on_mouse_move);
		window().add_event_listener(on_mouse_wheel);
		window().add_event_listener(on_context_menu);
		window().add_event_listener(on_key_down);
		window().add_event_listener(on_key_up);

		resize();

		update(0.0);
	}

	fn get_width(&self) -> u32 {
		self.width
	}
	fn get_height(&self) -> u32 {
		self.height
	}

	fn fill(&mut self, color: Colors) {
		self.absolute_mode(true);
		self.set_fill_style(color);
		self.ctx
			.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
		self.absolute_mode(false);
	}

	fn absolute_mode(&mut self, on: bool) {
		if on {
			self.ctx.save();
			self.ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
		} else {
			self.ctx.restore();
		}
	}

	fn draw_line<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, start: T, end: T2, color: Colors) {
		let (x, y) = start.into().into();
		let end: (f64, f64) = end.into().into();

		self.set_stroke_style(color);
		self.ctx.set_line_width(1.0 / game().mouse.scale() as f64);
		self.ctx.begin_path();
		self.ctx.move_to(x, y);
		self.ctx.line_to(end.0, end.1);
		self.ctx.stroke();
	}

	fn fill_rect<T: Into<GamePos>, T2: Into<GamePos>>(&mut self, pos: T, size: T2, color: Colors) {
		let (x, y) = pos.into().into();
		let size: (f64, f64) = size.into().into();

		self.set_fill_style(color);
		self.ctx.fill_rect(x, y, size.0, size.1);
	}
	fn stroke_rect<T: Into<GamePos>, T2: Into<GamePos>>(
		&mut self,
		pos: T,
		size: T2,
		line_width: f32,
		color: Colors,
	) {
		let (x, y) = pos.into().into();
		let size: (f64, f64) = size.into().into();

		self.set_stroke_style(color);
		self.ctx.set_line_width(line_width as f64);
		self.ctx.stroke_rect(x, y, size.0, size.1);
	}

	fn fill_circle<T: Into<GamePos>>(&mut self, pos: T, radius: f32, color: Colors) {
		let (x, y) = pos.into().into();

		self.set_fill_style(color);
		self.ctx.begin_path();
		self.ctx
			.arc(x, y, radius as f64, 0.0, 2.0 * std::f64::consts::PI, false);
		self.ctx.fill(Default::default());
	}
	fn stroke_circle<T: Into<GamePos>>(
		&mut self,
		pos: T,
		radius: f32,
		line_width: f32,
		color: Colors,
	) {
		let (x, y) = pos.into().into();

		self.set_stroke_style(color);
		self.ctx.set_line_width(line_width as f64);
		self.ctx.begin_path();
		self.ctx
			.arc(x, y, radius as f64, 0.0, 2.0 * std::f64::consts::PI, false);
		self.ctx.stroke();
	}

	fn draw_text<T: Into<GamePos>>(&mut self, text: &str, pos: T, color: Colors) {
		let (x, y) = pos.into().into();

		self.set_fill_style(color);
		self.ctx.fill_text(text, x, y + 4.0, None);
	}

	fn draw_asset<T: Into<GamePos>>(&mut self, (row, id): (usize, usize), pos: T) {
		let (x, y) = pos.into().into();

		#[rustfmt::skip]
		self.ctx.draw_image_s(
			self.assets.clone(),
			(id * TILE_SIZE) as f64, (row * TILE_SIZE) as f64,
			TILE_SIZE as f64, TILE_SIZE as f64,
			x, y,
			TILE_SIZE as f64, TILE_SIZE as f64
		).expect("Unable to draw image");
	}

	fn draw_background(&mut self) {
		js! {
			@{ &self.ctx }.drawImage(@{ &self.background_canvas }, 0.0, 0.0);
		}
	}

	fn clear_background(&mut self) {
		self.bg.fill_rect(
			0.0,
			0.0,
			self.background_canvas.width() as f64,
			self.background_canvas.height() as f64,
		);
	}

	fn draw_to_background<T: Into<GamePos>>(&mut self, (row, id): (usize, usize), pos: T) {
		let (x, y) = pos.into().into();

		#[rustfmt::skip]
		self.bg.draw_image_s(
			self.assets.clone(),
			(id * TILE_SIZE) as f64, (row * TILE_SIZE) as f64,
			TILE_SIZE as f64, TILE_SIZE as f64,
			x, y,
			TILE_SIZE as f64, TILE_SIZE as f64
		).expect("Unable to draw image");
	}
}

fn convert_key_code(key: &str) -> Option<ui::KeyCode> {
	use crate::world::Dir;
	use ui::KeyCode::*;

	match key {
		"Space" => Some(Space),
		"Escape" => Some(Escape),
		"Enter" => Some(Enter),
		"Backspace" => Some(Backspace),
		"Delete" => Some(Delete),
		"ArrowUp" => Some(Arrow(Dir::Up)),
		"ArrowDown" => Some(Arrow(Dir::Down)),
		"ArrowLeft" => Some(Arrow(Dir::Left)),
		"ArrowRight" => Some(Arrow(Dir::Right)),
		c if c.len() == 1 => {
			let c = c.chars().next().unwrap();
			if let Some(num) = c.to_digit(10) {
				Some(Number(num as usize))
			} else {
				Some(Letter(c))
			}
		}
		_ => None,
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {
	pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
		Color { r, g, b, a: 255 }
	}
	pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color { r, g, b, a }
	}
	pub fn to_css(self) -> String {
		format!(
			"rgba({}, {}, {}, {})",
			self.r,
			self.g,
			self.b,
			self.a as f64 / 255.0
		)
	}
}
