use super::map;

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
    pub player_offset: f32,
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
        offset: f32,
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
            player_offset: offset,
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

    pub fn check_turnaround(&mut self, map: map::Map) {
        if self.x_pos < map.l_wall.x + 20.0 || self.x_pos > map.r_wall.x - 20.0 {
            self.turn_and_run();
        }
    }

    pub fn check_wall_slide(&mut self, map: map::Map) -> bool {
        if self.sliding {
            return true;
        } else {
            if (self.x_pos > (map.l_wall.x + (map.l_wall.w / 2.0))
                && self.x_pos < ((map.l_wall.x + (map.l_wall.w / 2.0)) + 5.0))
                || (self.x_pos > (map.r_wall.x - (map.r_wall.w / 2.0) - 15.0)
                    && self.x_pos < map.r_wall.x - (map.r_wall.w / 2.0))
            {
                if !self.sliding {
                    self.x_velocity = 0.0;
                }
                return true;
            }
            if (self.x_pos > 680.0 && self.x_pos < 685.0)
                && (self.y_pos > map.platforms[0].y && self.y_pos < (map.platforms[0].y + map.platforms[0].h))
            {
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

    pub fn check_lower_platform(&mut self, map: map::Map) {
        if self.x_pos <= map.platforms[0].w {
            if (self.y_pos >= map.platforms[0].y) && (self.y_pos <= map.platforms[0].y + 20.0) {
                self.grounded = true;
                self.y_pos = map.platforms[0].y - 20.0;
            }
        }
    }

    pub fn turn_and_run(&mut self) {
        self.direction *= -1.0;
        self.x_velocity = X_VELOCITY * self.direction;
    }
}
