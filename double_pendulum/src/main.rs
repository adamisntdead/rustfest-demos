//! A Rust based simulation of the double pendulum,
//! written for RustFest Rome 2018

extern crate ggez;

use ggez::conf;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event;
use ggez::graphics::{self, DrawMode, Point2};
use std::env;
use std::path;

mod double_pendulum;

fn world_to_screen_coords(point: Point2) -> Point2 {
    Point2::new(point.x + 512.0, point.y + 512.0)
}

struct MainState {
    pendulum: double_pendulum::DoublePendulumLagrangian,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            pendulum: double_pendulum::DoublePendulumLagrangian {
                g: 9.8,
                m1: 10.0,
                m2: 5.0,
                t1: 2.0,
                t2: 1.5,
                dt1: 0.0,
                dt2: 0.0,
                l1: 200.0,
                l2: 150.0,
            },
        };
        Ok(s)
    }

    fn draw_bobs(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Calculate the first bob
        let x1 = self.pendulum.l1 * self.pendulum.t1.sin();
        let y1 = self.pendulum.l1 * self.pendulum.t1.cos();

        // Calculate the second bob
        let x2 = x1 + (self.pendulum.l2 * self.pendulum.t2.sin());
        let y2 = y1 + (self.pendulum.l2 * self.pendulum.t2.cos());

        // Draw the two bobs
        let bob1 = world_to_screen_coords(Point2::new(x1 as f32, y1 as f32));
        let bob2 = world_to_screen_coords(Point2::new(x2 as f32, y2 as f32));

        graphics::circle(ctx, DrawMode::Fill, bob1, self.pendulum.m1 as f32, 0.2)?;
        graphics::circle(ctx, DrawMode::Fill, bob2, self.pendulum.m2 as f32, 0.2)?;

        // Draw the two lines
        let origin = world_to_screen_coords(Point2::new(0.0, 0.0));
        graphics::line(ctx, &[origin, bob1], 2.0)?;
        graphics::line(ctx, &[bob1, bob2], 2.0)?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // let energy = double_pendulum::kinetic_energy()
        self.pendulum.time_step(0.05);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.draw_bobs(ctx)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let mut cb = ContextBuilder::new("double-pendulum", "adamisntdead")
        .window_setup(conf::WindowSetup::default().title("Double Pendulum"))
        .window_mode(conf::WindowMode::default().dimensions(1024, 1024));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build().unwrap();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = event::run(ctx, game);

            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }
}
