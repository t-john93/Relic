// use event::{run, EventHandler, KeyCode, KeyMods};
// use ggez::{conf, event, Context, ContextBuilder, GameResult};
// use std::env;
// use std::path;

// mod engine;
// mod map;
// mod view;

// pub const WIN_WIDTH: f32 = 1000.0;
// pub const WIN_HEIGHT: f32 = 600.0;

// pub struct PlayerState {
//     player_physics: engine::Engine,
//     map_model: map::Map,
//     resources: view::Resources,
//     win: bool,
// }

// impl PlayerState {
//     fn new(ctx: &mut Context) -> GameResult<PlayerState> {
//         let s = PlayerState {
//             player_physics: engine::Engine::construct_new(
//                 30.0,
//                 400.0,
//                 engine::X_VELOCITY,
//                 0.0,
//                 engine::GRAVITY,
//                 engine::DIRECTION,
//                 false,
//                 false,
//             ),
//             map_model: map::Map::construct_new(),
//             resources: view::Resources::new(ctx),
//             win: false,

//         };
//         Ok(s)
//     }

// }

// impl EventHandler for PlayerState {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult {
//         self.player_physics.check_ground(self.map_model);
//         self.player_physics.check_lower_platform(self.map_model);

//         if self.player_physics.sliding {
//             self.player_physics.get_sliding_y_pos();
//         } else {
//             self.player_physics.get_y_pos();
//             self.player_physics.get_x_pos();
//             self.player_physics.check_turnaround(self.map_model);
//         }

//         if (self.player_physics.x_pos < self.map_model.star_location.x)
//             && (self.player_physics.y_pos <= self.map_model.star_location.y)
//         {
//             self.win = true;
//         }

//         if self.player_physics.grounded {
//             self.player_physics.sliding = false;
//         } else {
//             self.player_physics.sliding = self.player_physics.check_wall_slide(self.map_model);
//         }

//         Ok(())
//     }

//     fn key_down_event(
//         &mut self,
//         ctx: &mut Context,
//         keycode: KeyCode,
//         _keymod: KeyMods,
//         _repeat: bool,
//     ) {
//         match keycode {
//             KeyCode::Space => {
//                 if self.player_physics.grounded {
//                     self.player_physics.y_velocity = engine::FREEFALL;
//                     self.player_physics.grounded = false;
//                 }

//                 if self.player_physics.sliding {
//                     self.player_physics.sliding = false;
//                     self.player_physics.turn_and_run();
//                     self.player_physics.y_velocity = engine::FREEFALL;
//                 }
//             }
//             KeyCode::Escape => event::quit(ctx),
//             _ => (),
//         }
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         if !self.win {
//             view::render_game(self, ctx)
//         } else {
//             view::render_win(self, ctx)
//         }
//     }
// }

// pub fn main() -> GameResult {
//     let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
//         let mut path = path::PathBuf::from(manifest_dir);
//         path.push("resources");
//         path
//     } else {
//         path::PathBuf::from("./resources")
//     };
//     let cb = ContextBuilder::new("relic", "ggez")
//         .add_resource_path(resource_dir)
//         .window_setup(conf::WindowSetup::default().title("Relic"))
//         .window_mode(conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT));
//     let (ctx, event_loop) = &mut cb.build()?;
//     let state = &mut PlayerState::new(ctx)?;


//     run(ctx, event_loop, state)
// }
extern crate ggez;
extern crate nalgebra;
extern crate ncollide;

use ggez::*;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{Color, DrawMode, Rect};
use nalgebra::{Isometry2, Vector2};
use ncollide::shape::{Cuboid2, ShapeHandle2};
use ncollide::world::{CollisionGroups, CollisionObjectHandle, CollisionWorld2, GeometricQueryType};
use player::Player;

pub mod player;

pub struct CollisionData {}

struct MainState {
	player: Player,
	world: CollisionWorld2<f32, ()>,
}

impl MainState {
	fn new(_ctx: &mut Context) -> Self {
		MainState {
			player: Player::new(),
			world: CollisionWorld2::new(0.02),
		}
	}

	pub fn add_collision_entity(&mut self, isometry: Isometry2<f32>, shape_handle:
	ShapeHandle2<f32>,
		groups: CollisionGroups, query: GeometricQueryType<f32>) -> CollisionObjectHandle {
		self.world.add(isometry, shape_handle, groups, query, ())
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		self.player.update(&mut self.world);
		self.world.update();

		if self.world.contacts().count() > 0 {
			self.player.set_color(Color::new(1., 0., 0., 1.));
		} else {
			self.player.set_color(Color::new(1., 1., 1., 1.));
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		graphics::set_color(ctx, Color::new(0., 1., 0., 1.));
		graphics::rectangle(ctx, DrawMode::Fill, Rect::new(50., 50., 32., 32.));
		self.player.draw(ctx);
		graphics::present(ctx);
		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, key: Keycode, _keymod: Mod, _repeat: bool) {
		if _repeat { return; }
		match key {
			Keycode::Z => self.player.velocity.y -= 1.,
			Keycode::S => self.player.velocity.y += 1.,
			Keycode::Q => self.player.velocity.x -= 1.,
			Keycode::D => self.player.velocity.x += 1.,
			_ => (),
		}
	}

	fn key_up_event(&mut self, ctx: &mut Context, key: Keycode, _keymod: Mod, _repeat: bool) {
		if _repeat { return; }
		match key {
			Keycode::Z => self.player.velocity.y += 1.,
			Keycode::S => self.player.velocity.y -= 1.,
			Keycode::Q => self.player.velocity.x += 1.,
			Keycode::D => self.player.velocity.x -= 1.,
			_ => (),
		}
	}
}

pub fn main() {
	let c = conf::Conf::new();
	let ctx = &mut Context::load_from_conf("ggez_ncollide_example", "opinon", c).unwrap();

	let mut state = MainState::new(ctx);
	let shape = ShapeHandle2::new(Cuboid2::new(Vector2::new(16., 16.)));
	let groups = CollisionGroups::new();
	let query = GeometricQueryType::Contacts(0., 0.);
	state.add_collision_entity(Isometry2::new(Vector2::new(50., 50.), 0.), shape.clone(), groups,
							   query);
	let player_collision_handle = state.add_collision_entity(Isometry2::new(Vector2::new(0., 0.), 0.), shape.clone(), groups, query);
	state.player.set_col_handle(player_collision_handle);

	event::run(ctx, &mut state).unwrap();
}