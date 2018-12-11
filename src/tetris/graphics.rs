extern crate ggez;

use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Drawable, Font, Point2, Text};
use std::collections::HashSet;
use tetris::grid::Grid;
use tetris::piece::{into_set, piece_color, Coord, Piece};

const PIECE_BOX_SIZE: i32 = 28;
const OUTER_BOX_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.8];
const INNER_BOX_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.8];

pub fn score(ctx: &mut Context, score: u32) -> GameResult<()> {
	let outer_box = graphics::Rect::new_i32(600, 40, 200, 50);
	let inner_box = graphics::Rect::new_i32(605, 45, 190, 40);
	graphics::set_color(ctx, OUTER_BOX_COLOR.into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_box)?;
	graphics::set_color(ctx, INNER_BOX_COLOR.into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, inner_box)?;
	graphics::set_color(ctx, [0.0, 0.0, 0.0, 1.0].into())?;
	let font = Font::default_font().expect("Could not get default font");
	let text = Text::new(ctx, &score.to_string(), &font)?;
	text.draw(ctx, Point2::new(640.0, 55.0), 0.0)
}

pub fn next_piece(ctx: &mut Context, piece: Piece) -> GameResult<()> {
	let outer_box = graphics::Rect::new_i32(600, 180, 200, 200);
	let inner_box = graphics::Rect::new_i32(605, 185, 190, 190);
	graphics::set_color(ctx, OUTER_BOX_COLOR.into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_box)?;
	graphics::set_color(ctx, INNER_BOX_COLOR.into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, inner_box)?;
	draw_piece(ctx, piece, 610, 190)
}

pub fn grid(ctx: &mut Context, grid: &Grid) -> GameResult<()> {
	let outer_box = graphics::Rect::new_i32(75, 35, 400, 800);
	let inner_box = graphics::Rect::new_i32(80, 40, 390, 790);
	graphics::set_color(ctx, OUTER_BOX_COLOR.into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_box)?;
	graphics::set_color(ctx, INNER_BOX_COLOR.into())?;
	graphics::rectangle(ctx, graphics::DrawMode::Fill, inner_box)
}

fn draw_piece(ctx: &mut Context,
			  piece: Piece,
			  x_axis: i32,
			  y_axis: i32) -> GameResult<()> {
	graphics::set_color(ctx, piece_color(piece).into())?;
	for coord in into_set(piece) {
		let rect = graphics::Rect::new_i32(coord.x as i32, coord.y as i32, PIECE_BOX_SIZE, PIECE_BOX_SIZE);
		graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
	}
	Ok(())
}
