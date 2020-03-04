pub struct EngineState {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub gravity: f32,
    pub grounded: bool,
}

impl EngineState {
    pub fn new() -> EngineState {
        let es = EngineState {
            x_pos: 0.0,
            y_pos: 0.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            gravity: 0.0,
            grounded: false,
        };
        es
    }
    pub fn construct_new(x: f32,y: f32,xv: f32,yv: f32,g: f32,on_ground: bool) -> EngineState {
        let es = EngineState {
            x_pos: x,
            y_pos: y,
            x_velocity: xv,
            y_velocity: yv,
            gravity: g,
            grounded: on_ground,
        };
        es
    }
    pub fn get_x_pos(&mut self) {
        self.x_pos += self.x_velocity;
        self.y_velocity += self.gravity;
        if self.x_pos < 50.0 || self.x_pos > 750.0 { self.x_velocity *= -1.0; }
    }
    pub fn get_y_pos(&mut self) {
        if self.check_ground() {
            self.y_pos = 500.0;
            self.y_velocity = 0.0;
        }
        self.y_pos += self.y_velocity;
    }
    pub fn check_ground(&mut self) -> bool {
        if self.y_pos > 500.0 { self.grounded = true; }
        self.grounded
    }
    // pub fn set_y_velocity(&mut self, velocity: f32) {
    //     self.y_velocity = velocity;
    // }
    // pub fn set_grounded(&mut self, on_ground: bool) {
    //     self.grounded = on_ground;
    // }
}
