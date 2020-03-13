use ggez::graphics;
// use ggez::{graphics, nalgebra as na};

#[derive(Clone, Copy)]
pub struct Map {
    pub ground: graphics::Rect,
    pub ceiling: graphics::Rect,
    pub l_wall: graphics::Rect,
    pub r_wall: graphics::Rect,
    // pub obstacles: Obstacles,
}

impl Map {
    pub fn construct_new(//obs: Vec<graphics::Rect>, // win_width: f32,
        // win_height: f32,
    ) -> Map {
        let map = Map {
            ground: graphics::Rect::new(0.0, 520.0, 820.0, 80.0),
            ceiling: graphics::Rect::new(0.0, 0.0, 820.0, 50.0),
            l_wall: graphics::Rect::new(0.0, 0.0, 40.0, 800.0),
            r_wall: graphics::Rect::new(820.0, 0.0, 50.0, 800.0),
        };
        map
    }
}
#[derive(Clone)]
pub struct Obstacles {
    pub rectangles: Vec<graphics::Rect>,
    pub star_location: (f32, f32), //Supposed to be a Point2, but having an errors
                                   // pub star_location: na::Point(f32, f32),
                                   // pub star_location_X: na::Point,
                                   // pub star_location_Y: na::geometry::Point,
}

impl Obstacles {
    pub fn map1() -> Obstacles {
        let map_1 = Obstacles {
            rectangles: [
                graphics::Rect::new(0.0, 300.0, 700.0, 75.0), //Lower block
                graphics::Rect::new(0.0, 200.0, 100.0, 100.0),
            ]
            .to_vec(),
            star_location: (50.0, 150.0),
            // star_location: na::Point2::new(50.0, 100.0),
        };
        map_1
    }
}
