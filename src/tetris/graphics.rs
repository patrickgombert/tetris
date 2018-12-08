extern crate ggez;

use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Drawable, Font, Point2, Text};

pub fn score(ctx: &mut Context, score: u32) -> GameResult<()> {
	let outer_box = graphics::Rect::new_i32(600, 40, 200, 50);
	let inner_box = graphics::Rect::new_i32(605, 45, 190, 40);
	graphics::set_color(ctx, [0.0, 0.0, 0.0, 0.8].into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_box)?;
	graphics::set_color(ctx, [1.0, 1.0, 1.0, 0.8].into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, inner_box)?;
	graphics::set_color(ctx, [0.0, 0.0, 0.0, 1.0].into())?;
	let font = Font::default_font().expect("Could not get default font");
	let text = Text::new(ctx, &score.to_string(), &font)?;
	text.draw(ctx, Point2::new(640.0, 55.0), 0.0)?;
	Ok(())
}
