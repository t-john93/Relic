use crate::PlayerState;
use ggez::{graphics, nalgebra as na, Context, GameResult};

pub fn render_game(player_state: &mut PlayerState, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

    let ground = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        player_state.map_model.ground,
        graphics::BLACK,
    )?;
    let ceiling = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        player_state.map_model.ceiling,
        graphics::BLACK,
    )?;
    let l_wall = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        player_state.map_model.l_wall,
        graphics::BLACK,
    )?;
    let r_wall = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        player_state.map_model.r_wall,
        graphics::BLACK,
    )?;

    //Drawing the Obstacles
    let mut i = 0;
    while i < player_state.map_model.platforms.len() {
        let obs = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_state.map_model.platforms[i],
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &obs , (na::Point2::new(0.0, 0.0),))?;
        i = i + 1;
    }
    
    if player_state.player_physics.direction > 0.0 {
        graphics::draw(
            ctx,
            &player_state.resources.character_sprite[0],
            (na::Point2::new(
                player_state.player_physics.x_pos,
                player_state.player_physics.y_pos - 32.0,
            ),),
        )?;
    } else {
        graphics::draw(
            ctx,
            &player_state.resources.character_sprite[1],
            (na::Point2::new(
                player_state.player_physics.x_pos,
                player_state.player_physics.y_pos - 32.0,
            ),),
        )?;
    }

    // Draw Border
    graphics::draw(ctx, &ground, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &ceiling, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &l_wall, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &r_wall, (na::Point2::new(0.0, 0.0),))?;
    graphics::present(ctx)?;

    // Draw Star
    graphics::draw(ctx, &player_state.resources.star, (na::Point2::new(player_state.map_model.star_location.0, player_state.map_model.star_location.1),))?;
    Ok(())
}
