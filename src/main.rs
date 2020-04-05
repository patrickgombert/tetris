extern crate ggez;

mod tetris;

use ggez::event::Keycode;
use ggez::{event, graphics, Context, GameResult};
use std::time::Instant;
use tetris::grid::Grid;

struct GameState {
    score: u32,
    level: u32,
    update_duration: u128,
    last_update: Instant,
    grid: Grid,
}

impl GameState {
    fn new() -> Self {
        GameState {
            score: 0,
            level: 0,
            update_duration: 2000,
            last_update: Instant::now(),
            grid: Grid::new(20, 10),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let duration = Instant::now().duration_since(self.last_update);
        if duration.as_millis() > self.update_duration {
            let dead_lines = self.grid.tick();
            let line_score = match dead_lines {
                0 => 0,
                1 => 40 * (self.level + 1),
                2 => 100 * (self.level + 1),
                3 => 300 * (self.level + 1),
                4 => 1400 * (self.level + 1),
                _ => panic!("cleared more than 4 lines"),
            };
            self.score = self.score + line_score;
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, [0.4, 0.1, 0.1, 1.0].into());
        tetris::graphics::score(ctx, self.score)?;
        tetris::graphics::next_piece(ctx, self.grid.next_piece)?;
        tetris::graphics::grid(ctx, &self.grid)?;
        tetris::graphics::state(ctx, &self.grid.state)?;
        graphics::present(ctx);
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: event::Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::D => self.grid.rotate_right(),
            Keycode::A => self.grid.rotate_left(),
            Keycode::Down => {
                self.grid.down();
                ()
            }
            Keycode::Left => self.grid.left(),
            Keycode::Right => self.grid.right(),
            Keycode::Space => self.grid.drop(),
            _ => (),
        }
    }
}

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("tetris", "Patrick Gombert")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(900, 900))
        .build()
        .unwrap();
    match event::run(ctx, &mut GameState::new()) {
        Err(e) => println!("Error {}", e),
        Ok(_) => (),
    }
}
