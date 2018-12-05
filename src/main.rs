extern crate ggez;

mod tetris;

use ggez::event::Keycode;
use ggez::{event, graphics, Context, GameResult};
use tetris::grid::Grid;

struct GameState {
    score: i32,
    grid: Grid,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            grid: Grid::new(20, 10)
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {

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
