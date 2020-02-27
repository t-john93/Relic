use ggez::{
    Context,
    graphics::{ 
//        Font,
        Image,
//        spritebatch::SpriteBatch,
    }
};

pub struct Resources {
    pub character_sprite: Image,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> Resources {
        let character_sprite = Image::new(ctx, "/images/character_sprite.png").unwrap();

        Resources {
            character_sprite: character_sprite.clone(),
        }
    }
}
