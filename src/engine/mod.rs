use ggez::graphics::Rect;

pub struct Engine {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub gravity: f32,
    pub grounded: bool,
}

impl Engine{
    pub fn construct_new(x: f32,y: f32,xv: f32,yv: f32,g: f32,on_ground: bool) -> Engine {
        let engine = Engine {
            x_pos: x,
            y_pos: y,
            x_velocity: xv,
            y_velocity: yv,
            gravity: g,
            grounded: on_ground,
        };
        engine
    }
    pub fn get_x_pos(&mut self) {
        self.x_pos += self.x_velocity;
        self.y_velocity += self.gravity;
    }
    pub fn get_y_pos(&mut self) {
        if self.grounded {
            self.y_pos = 500.0;
            self.y_velocity = 0.0;
        }
        self.y_pos += self.y_velocity;
    }
    pub fn check_wall(&mut self, _map: Rect) {
        if self.x_pos < 20.0 || self.x_pos > 780.0 { self.x_velocity *= -1.0; }
    }
    pub fn check_ground(&mut self, map: Rect) -> bool {
        if self.y_pos >= map.top() { self.grounded = true; }
        self.grounded
    }
}
