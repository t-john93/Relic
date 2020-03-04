use ggez::{Context, ContextBuilder, GameResult, graphics, event, nalgebra as na};
use event::{EventHandler, run, KeyCode, KeyMods};
use graphics::Rect;
// use conf::{WindowSetup};
use std::env;
use std::path;

// mod engine;

struct MainState {
    x_pos: f32,
    y_pos: f32,
    x_velocity: f32,
    y_velocity: f32,
    gravity: f32,
    grounded: bool,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {x_pos: 100.0, y_pos: 200.0, x_velocity: 3.0, y_velocity: 0.0, gravity: 0.5, grounded: false};
        Ok(s)
    }
}

impl EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.x_pos += self.x_velocity;
        self.y_velocity += self.gravity;
        if self.y_pos > 500.0 {
            self.y_pos = 500.0;
            self.y_velocity = 0.0;
            self.grounded = true;
        }
        self.y_pos += self.y_velocity;
        if self.x_pos < 50.0 || self.x_pos > 750.0 { self.x_velocity *= -1.0 }

        // BROKEN
        // engine::get_x_pos(&self);
        // engine::get_y_pos(self.y_pos, self.y_velocity, self.gravity, self.grounded);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => {

                if self.grounded {
                    self.y_velocity = -12.0;
                    self.grounded = false;
                }
            }
            KeyCode::Escape => {
                event::quit(ctx)
            }
            _ => ()
        }
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // let ground = Rect::zero();
        let ground_rec = Rect::new(0.0, 520.0, 800.0, 100.0);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.x_pos, self.y_pos),
            20.0,
            0.5,
            graphics::BLACK,
        )?;
        let ground = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ground_rec,
            graphics::WHITE
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &ground, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult { 

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("Relic", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    run(ctx, event_loop, state)
}