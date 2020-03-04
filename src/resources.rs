use ggez::{
    Context,
    graphics::{ 
        Image,
    },
};

pub struct Resources {
    pub character_sprite: Vec<Image>,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> Resources {
        
        let mut resources: Resources = Resources {
            character_sprite: Vec::new(),
        }; 

        resources.character_sprite.insert(0, Image::new(ctx, "/images/character_sprite_right.png").unwrap());
        resources.character_sprite.insert(1, Image::new(ctx, "/images/character_sprite_left.png").unwrap());
        resources.character_sprite.insert(2, Image::new(ctx, "/images/character_sprite_jump.png").unwrap());

        return resources;
    }
}
