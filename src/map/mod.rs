use ggez::graphics;
// use ggez::{graphics, nalgebra as na};

#[derive(Clone, Copy)]
pub struct Map {
    pub ground: graphics::Rect,
    pub ceiling: graphics::Rect,
    pub l_wall: graphics::Rect,
    pub r_wall: graphics::Rect,
    pub platforms: [graphics::Rect;2],
    pub star_location: (f32,f32),//Supposed to be a Point2, but having an errors

    // pub obstacles: Obstacles,
}

impl Map {
    pub fn construct_new(
    ) -> Map {
        let map = Map {
            // TODO: read map values from file
            ground: graphics::Rect::new(0.0, 520.0, 820.0, 80.0),
            ceiling: graphics::Rect::new(0.0, 0.0, 820.0, 50.0),
            l_wall: graphics::Rect::new(0.0, 0.0, 40.0, 800.0),
            r_wall: graphics::Rect::new(820.0, 0.0, 50.0, 800.0),
            platforms: [
                graphics::Rect::new(0.0, 300.0, 700.0, 75.0),
                graphics::Rect::new(0.0, 200.0, 100.0, 100.0),
            ],
            star_location: (750.0, 477.0), //50, 150
            // star_location: na::Point2::new(50.0, 100.0),
        };
        map
    }
}