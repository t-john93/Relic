use super::map;

pub const MIN_X_POS: f32 = 20.0;
pub const MAX_X_POS: f32 = 780.0;
pub const MIN_Y_POS: f32 = 500.0;
pub const X_VELOCITY: f32 = 3.0;
pub const GRAVITY: f32 = 0.5;
pub const FREEFALL: f32 = -10.0;
pub const DIRECTION: f32 = 1.0;

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
        if self.grounded {
            self.y_velocity = 0.0;
        } else {
            self.y_pos += self.y_velocity;
        }
    }

    pub fn get_sliding_y_pos(&mut self) {
        self.y_pos += self.gravity;
    }

    pub fn check_turnaround(&mut self, _map: map::Map) {
        if self.x_pos < MIN_X_POS || self.x_pos > MAX_X_POS {
            self.turn_and_run();
        }
    }

    pub fn check_wall_slide(&mut self, map: map::Map) -> bool {
        if self.sliding {
            return true;
        } else {
            if (self.x_pos > MIN_X_POS && self.x_pos < (MIN_X_POS + 5.0))
                || (self.x_pos > (MAX_X_POS - 5.0) && self.x_pos < MAX_X_POS)
            {
                if !self.sliding {
                    self.x_velocity = 0.0;
                }
                return true;
            }
            if (self.x_pos > 680.0 && self.x_pos < 685.0)
                && (self.y_pos > 300.0 && self.y_pos < 350.0)
            {
                if !self.sliding {
                    self.x_velocity = 0.0;
                }
                return true;
            }
            if  (230.0..235.0).contains(&(self.x_pos - 16.))
                && (map.platforms[1].top()..map.platforms[1].bottom()+10.0).contains(&(self.y_pos)) {
                    if !self.sliding {
                        self.x_velocity = 0.0;
                    }
                return true;
            }
            if (265.0..270.).contains(&(self.x_pos + 16.))
                && (map.platforms[1].top()..map.platforms[1].bottom()+10.0).contains(&(self.y_pos)) {
                    if !self.sliding {
                        self.x_velocity = 0.0;
                    }
                return true;
            }
        }
        return false;
    }

    pub fn check_ground(&mut self, map: map::Map) {
        if self.y_pos >= map.ground.top() {
            self.grounded = true;
            self.y_pos = MIN_Y_POS;
        }
        if self.grounded && self.x_velocity == 0.0 {
            self.turn_and_run();
        }
    }

    pub fn check_lower_platform(&mut self, _map: map::Map) {
        if self.x_pos <= 700.0 {
            if (self.y_pos >= 300.0) && (self.y_pos <= 320.0) {
                self.grounded = true;
                self.y_pos = 280.0;
            }
        }
    }

    pub fn turn_and_run(&mut self) {
        self.direction *= -1.0;
        self.x_velocity = X_VELOCITY * self.direction;
    }
}
