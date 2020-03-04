pub get_x_pos(&mut state: MainState) {
    state.x_pos += state.x_velocity;
    state.y_velocity += state.gravity;

    if self.x_pos < 50.0 || self.x_pos > 750.0 { self.x_velocity *= -1.0 }
}

pub get_y_pos(&mut y_pos: f32, &mut y_velocity, &mut gravity: f32, &mut grounded: bool) {
        if y_pos > 500.0 {
            y_pos = 500.0;
            y_velocity = 0.0;
            grounded = true;
        }

        y_pos += y_velocity;
}