use ggez::{Context, ContextBuilder, GameResult, graphics, event, nalgebra as na, conf};
use event::{EventHandler, run, KeyCode, KeyMods};
use std::env;
use std::path;

mod engine;

pub struct PlayerState {
    player_physics: engine::Engine,
    // map_model: [graphics::Rect; 20],     // TODO return array of map polygons
    map_model: graphics::Rect,
    window: [f32; 2],
}

impl PlayerState {
    fn new() -> GameResult<PlayerState> {
        let s = PlayerState {
            player_physics: engine::Engine::construct_new(100.0, 200.0, 3.0, 0.0, 0.5, false),
            map_model: graphics::Rect::new(0.0, 520.0, 800.0, 80.0),
            window: [1000.0, 600.0],
        };
        Ok(s)
    }
}

impl EventHandler for PlayerState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player_physics.check_wall(self.map_model);
        self.player_physics.get_x_pos();
        self.player_physics.get_y_pos();
        self.player_physics.check_ground(self.map_model);
        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => {
                if self.player_physics.grounded {
                    self.player_physics.y_velocity = -12.0;
                    self.player_physics.grounded = false;
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
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.player_physics.x_pos, self.player_physics.y_pos),
            20.0,
            0.5,
            graphics::BLACK,
        )?;
        let ground = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.map_model,
            graphics::WHITE
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &ground, (na::Point2::new(0.0, 0.0),))?;
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
    let state = &mut PlayerState::new()?;
    let cb = ContextBuilder::new("relic", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Relic"))
        .window_mode(conf::WindowMode::default().dimensions(state.window[0], state.window[1]));
    let (ctx, event_loop) = &mut cb.build()?;
    run(ctx, event_loop, state)
}