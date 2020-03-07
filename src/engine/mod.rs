use ggez::graphics::Rect;

// Make constants for physics values
const MIN_X_POS: f32 = 20.0;
const MAX_X_POS: f32 = 780.0;
const MIN_Y_POS: f32 = 500.0;
const X_VELOCITY: f32 = 3.0;
const GRAVITY: f32 = 0.5;

pub struct Engine {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub gravity: f32,
    pub direction: f32,
    pub grounded: bool,
    pub sliding: bool,
}

impl Engine {
    pub fn construct_new(
        x: f32,
        y: f32,
        xv: f32,
        yv: f32,
        g: f32,
        dir: f32,
        on_ground: bool,
        slide: bool,
    ) -> Engine {
        let engine = Engine {
            x_pos: x,
            y_pos: y,
            x_velocity: xv,
            y_velocity: yv,
            gravity: g,
            direction: dir,
            grounded: on_ground,
            sliding: slide,
        };
        engine
    }

    pub fn get_x_pos(&mut self) {
        self.x_pos += self.x_velocity;
        self.y_velocity += self.gravity;
    }

    pub fn get_y_pos(&mut self) {
        if self.sliding {
            self.y_pos += self.gravity;
        } else {
            if self.grounded {
                self.y_pos = MIN_Y_POS;
                self.y_velocity = 0.0;
            }
            self.y_pos += self.y_velocity;
        }
    }

    pub fn check_turnaround(&mut self, _map: Rect) {
        if self.x_pos < MIN_X_POS || self.x_pos > MAX_X_POS {
            self.turn_and_run();
        }
    }

    pub fn check_wall_slide(&mut self, _map: Rect) -> bool {
        if (!self.grounded)
            && ((self.x_pos > MIN_X_POS && self.x_pos < (MIN_X_POS + 5.0))
                || (self.x_pos > (MAX_X_POS - 5.0) && self.x_pos < MAX_X_POS))
        {
            self.x_velocity = 0.0;
            return true;
        }
        self.gravity = GRAVITY;
        return false;
    }

    pub fn check_ground(&mut self, map: Rect) -> bool {
        if self.y_pos >= map.top() {
            self.grounded = true;
        }
        if self.x_velocity == 0.0 {
            self.turn_and_run();
        }
        self.grounded
    }

    pub fn turn_and_run(&mut self) {
        self.direction *= -1.0;
        self.x_velocity = X_VELOCITY * self.direction;
    }
}
