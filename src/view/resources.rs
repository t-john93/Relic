use ggez::{graphics::Image, Context};

pub struct Resources {
    pub character_sprite: Vec<Image>,
    pub star: Image,
    pub game_over: Image,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> Resources {
        let mut resources: Resources = Resources {
            character_sprite: Vec::new(),
            star: Image::new(ctx, "/images/star.png").unwrap(),
            game_over: Image::new(ctx, "/images/image.png").unwrap(),
        };

        resources.character_sprite.insert(
            0,
            Image::new(ctx, "/images/character_sprite_right.png").unwrap(),
        );
        resources.character_sprite.insert(
            1,
            Image::new(ctx, "/images/character_sprite_left.png").unwrap(),
        );

        return resources;
    }
}
