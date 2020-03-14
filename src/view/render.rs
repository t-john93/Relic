use crate::PlayerState;
use ggez::{graphics, nalgebra as na, Context, GameResult};

pub fn render_game(player_state: &mut PlayerState, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

    // let string: String = "Space to jump, Reach star to win";
    //create instructions
    let instructions = graphics::Text::new(
        // ctx,
        "Space to jump, Reach star to win".to_string(),
        // &TimesNewRoman,
    );

    // Create border
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

    // Draws the platforms
    let mut i = 0;
    while i < player_state.map_model.platforms.len() {
        let obs = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_state.map_model.platforms[i],
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &obs, (na::Point2::new(0.0, 0.0),))?;
        i = i + 1;
    }

    // Draw the star
    graphics::draw(
        ctx,
        &player_state.resources.star,
        (na::Point2::new(
            player_state.map_model.star_location.x,
            player_state.map_model.star_location.y,
        ),),
    )?;

    // Draw the character position
    if player_state.player_physics.direction > 0.0 {
        graphics::draw(
            ctx,
            &player_state.resources.character_sprite[0],
            (na::Point2::new(
                player_state.player_physics.x_pos,
                player_state.player_physics.y_pos - player_state.player_physics.player_offset,
            ),),
        )?;
    } else {
        graphics::draw(
            ctx,
            &player_state.resources.character_sprite[1],
            (na::Point2::new(
                player_state.player_physics.x_pos,
                player_state.player_physics.y_pos - player_state.player_physics.player_offset,
            ),),
        )?;
    }

    // Draw Border
    graphics::draw(ctx, &ground, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &ceiling, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &l_wall, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &r_wall, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &instructions, (na::Point2::new(0.0,0.0),))?;
    graphics::present(ctx)?;

    Ok(())
}

pub fn render_win(player_state: &mut PlayerState, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);

    graphics::draw(
        ctx,
        &player_state.resources.game_over,
        (na::Point2::new(0.0, 0.0),),
    )?;
    graphics::present(ctx)?;
    Ok(())
}
