
use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra as na;


struct MainState {
    pos_x: f32,
    pos_y: f32,
    jump: bool,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y:400.0, jump:false,};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.jump {
            self.jump = false;
            self.pos_y = self.pos_y + 100.0;
        }  
        if keyboard::is_key_pressed(_ctx, KeyCode::Space) {
            self.pos_y = self.pos_y - 100.0;
            self.jump = true;
        }
        if self.pos_x < 750.0  {
            self.pos_x = self.pos_x + 0.5;
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, self.pos_y),
            50.0,
            2.0,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}