use ggez::graphics;

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

            // create interior structure
            // obstacles: vec![             
            //     graphics::Rect::new(500.0, 200.0, 50.0, 0.0),
            //     graphics::Rect::new(100.0, 10.0, 100.0, 10.0),
            // ],
        };
        map
    }
}
#[derive(Clone)]
pub struct Obstacles{
    pub rectangles: Vec<graphics::Rect>,
}

impl Obstacles {
    pub fn map1() -> Obstacles{
        let map_1 = Obstacles {
            rectangles: [
                graphics::Rect::new(500.0, 200.0, 50.0, 10.0),
                graphics::Rect::new(100.0, 10.0, 50.0, 10.0),
            ].to_vec(),
        };
        map_1
    }



}