extern crate ggez;

use ggez::graphics::{Drawable, Font, Point2, Text};
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;
use tetris::grid::Grid;
use tetris::piece::{into_set, Coord, Piece};

const PIECE_BOX_SIZE: i32 = 39;
const OUTER_BOX_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.5];
const INNER_BOX_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.6];
const DEAD_BOX_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

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
    draw_piece(ctx, piece, 500, 280, false)
}

pub fn grid(ctx: &mut Context, grid: &Grid) -> GameResult<()> {
    let outer_box = graphics::Rect::new_i32(75, 35, 400, 800);
    let inner_box = graphics::Rect::new_i32(80, 40, 390, 790);
    graphics::set_color(ctx, OUTER_BOX_COLOR.into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_box)?;
    graphics::set_color(ctx, INNER_BOX_COLOR.into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, inner_box)?;
    graphics::set_color(ctx, [0.0, 0.0, 0.0, 1.0].into())?;
    for i in (80..470).step_by(PIECE_BOX_SIZE as usize) {
        graphics::line(
            ctx,
            &[Point2::new(i as f32, 40.0), Point2::new(i as f32, 830.0)],
            1.0,
        )?;
    }
    draw_piece(ctx, grid.piece, 80, 40, true)
}

pub fn state(ctx: &mut Context, state: &HashSet<Coord>) -> GameResult<()> {
    for piece in state {
        let x = 80 + (piece.x as i32) * PIECE_BOX_SIZE;
        let y = 40 + (piece.y as i32) * PIECE_BOX_SIZE;
        let outer_rect = graphics::Rect::new_i32(x, y, PIECE_BOX_SIZE, PIECE_BOX_SIZE);
        graphics::set_color(ctx, OUTER_BOX_COLOR.into())?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_rect)?;
        let rect = graphics::Rect::new_i32(x + 2, y + 2, PIECE_BOX_SIZE - 4, PIECE_BOX_SIZE - 4);
        graphics::set_color(ctx, DEAD_BOX_COLOR.into())?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
    }
    Ok(())
}

fn draw_piece(
    ctx: &mut Context,
    piece: Piece,
    x_axis: i32,
    y_axis: i32,
    truncate: bool,
) -> GameResult<()> {
    let color = piece_color(piece).into();
    for coord in into_set(&piece) {
        if !truncate || coord.y >= 0 {
            let x = x_axis + ((coord.x as i32) * PIECE_BOX_SIZE);
            let y = y_axis + ((coord.y as i32) * PIECE_BOX_SIZE);
            graphics::set_color(ctx, OUTER_BOX_COLOR.into())?;
            let outer_rect = graphics::Rect::new_i32(x, y, PIECE_BOX_SIZE, PIECE_BOX_SIZE);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, outer_rect)?;
            graphics::set_color(ctx, color)?;
            let rect =
                graphics::Rect::new_i32(x + 2, y + 2, PIECE_BOX_SIZE - 4, PIECE_BOX_SIZE - 4);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
        }
    }
    Ok(())
}

const I_COLOR: [f32; 4] = [0.6, 0.8, 0.9, 1.0];
const O_COLOR: [f32; 4] = [0.9, 0.8, 0.0, 1.0];
const T_COLOR: [f32; 4] = [0.9, 0.2, 1.0, 1.0];
const S_COLOR: [f32; 4] = [0.6, 0.8, 0.5, 1.0];
const Z_COLOR: [f32; 4] = [0.7, 0.1, 0.2, 1.0];
const J_COLOR: [f32; 4] = [0.4, 0.1, 1.0, 1.0];
const L_COLOR: [f32; 4] = [0.9, 0.5, 0.0, 1.0];

fn piece_color(piece: Piece) -> [f32; 4] {
    match piece {
        Piece::I(_) => I_COLOR,
        Piece::O(_) => O_COLOR,
        Piece::T(_) => T_COLOR,
        Piece::S(_) => S_COLOR,
        Piece::Z(_) => Z_COLOR,
        Piece::J(_) => J_COLOR,
        Piece::L(_) => L_COLOR,
    }
}
