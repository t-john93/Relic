use event::{run, EventHandler, KeyCode, KeyMods};
use ggez::{conf, event, Context, ContextBuilder, GameResult};
use std::env;
use std::path;

mod engine;
mod view;
mod map;

pub const WIN_WIDTH: f32 = 1000.0;
pub const WIN_HEIGHT: f32 = 600.0;

pub struct PlayerState {
    player_physics: engine::Engine,
    map_model: map::Map,
    resources: view::Resources,
}

impl PlayerState {
    fn new(ctx: &mut Context) -> GameResult<PlayerState> {
        let s = PlayerState {
            player_physics: engine::Engine::construct_new(
                100.0,
                200.0,
                engine::X_VELOCITY,
                0.0,
                engine::GRAVITY,
                engine::DIRECTION,
                false,
                false,
            ),
            map_model: map::Map::construct_new(),
            resources: view::Resources::new(ctx),
        };
        Ok(s)
    }
}

impl EventHandler for PlayerState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player_physics.check_ground(self.map_model);
        self.player_physics.sliding = self.player_physics.check_wall_slide(self.map_model);
        if !self.player_physics.sliding {
            self.player_physics.check_turnaround(self.map_model);
        }
        self.player_physics.get_x_pos();
        self.player_physics.get_y_pos();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => {
                if self.player_physics.grounded {
                    self.player_physics.y_velocity = engine::FREEFALL;
                    self.player_physics.grounded = false;
                }

                if self.player_physics.sliding {
                    self.player_physics.turn_and_run();
                    self.player_physics.y_velocity = engine::FREEFALL;
                }
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        view::render_game(self, ctx)
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
    let cb = ContextBuilder::new("relic", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Relic"))
        .window_mode(conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut PlayerState::new(ctx)?;
    run(ctx, event_loop, state)
}
