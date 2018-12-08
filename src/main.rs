#![feature(duration_as_u128)]
extern crate ggez;

mod tetris;

use ggez::event::Keycode;
use ggez::{event, graphics, Context, GameResult};
use std::time::Instant;
use tetris::grid::Grid;
use tetris::graphics::*;

struct GameState {
    score: u32,
    update_duration: u128,
    last_update: Instant,
    grid: Grid,
}

impl GameState {
    fn new() -> Self {
        GameState {
            score: 0,
            update_duration: 2000,
            last_update: Instant::now(),
            grid: Grid::new(20, 10)
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let duration = Instant::now().duration_since(self.last_update);
        if duration.as_millis() > self.update_duration {
            let score = self.grid.tick();
            self.score = self.score + score;
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
		graphics::set_background_color(ctx, [0.6, 0.6, 0.9, 1.0].into());
        tetris::graphics::score(ctx, self.score)?;
		graphics::present(ctx);
		ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {
        match keycode {
            Keycode::Up => self.grid.rotate(),
            Keycode::Down => self.grid.down(),
            Keycode::Left => self.grid.left(),
            Keycode::Right => self.grid.right(),
            Keycode::Space => self.grid.drop(),
            _ => ()
        }
    }
}

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("tetris", "Patrick Gombert")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(900, 900))
        .build().unwrap();
    match event::run(ctx, &mut GameState::new()) {
        Err(e) => println!("Error {}", e),
        Ok(_) => ()
    }
}
