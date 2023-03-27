use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone)]
enum MovementDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

impl MovementDirection {
    fn as_request_index(&self) -> usize {
        match self {
            MovementDirection::Forward => 0,
            MovementDirection::Backward => 1,
            MovementDirection::Left => 2,
            MovementDirection::Right => 3,
            MovementDirection::Up => 4,
            MovementDirection::Down => 5,
        }
    }
}

pub struct Camera {
    position: [f32; 3],
    yaw: f32,
    pitch: f32,
    movement_request: [f32; 6],
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: [0.0, 0.0, 0.0],
            yaw: -90.0,
            pitch: 0.0,
            movement_request: [0.0; 6],
        }
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    fn set_move_direction(&mut self, direction: MovementDirection, amount: f32) {
        let index = direction.as_request_index();
        self.movement_request[index] = amount;
    }

    pub fn move_view(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw += x_offset;
        self.pitch = (self.pitch + y_offset).clamp(-89.0, 89.0);
    }

    pub fn get_yaw_pitch(&self) -> (f32, f32) {
        (self.yaw, self.pitch)
    }

    pub fn set_yaw_pitch(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
    }

    pub fn update_position(&mut self, frame_duration: std::time::Duration, sensitivity: f32) {
        let front = glm::normalize(&glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        let up = glm::vec3(0.0, 1.0, 0.0);

        let right = glm::normalize(&glm::cross(&front, &up));

        let calculate_factor = |start: MovementDirection, finish: MovementDirection| {
            self.movement_request[start.as_request_index()]
                - self.movement_request[finish.as_request_index()]
        };

        let mut position_diff = glm::Vec3::default();
        position_diff +=
            right * calculate_factor(MovementDirection::Right, MovementDirection::Left);
        position_diff +=
            front * calculate_factor(MovementDirection::Forward, MovementDirection::Backward);
        position_diff += up * calculate_factor(MovementDirection::Up, MovementDirection::Down);

        let secs_since_last_update = frame_duration.as_secs_f32();
        let position_diff = sensitivity * secs_since_last_update * position_diff;
        let position = glm::make_vec3(&self.position) + position_diff;
        self.position = [position[0], position[1], position[2]];
    }

    pub fn calculate_view(&self) -> [[f32; 4]; 4] {
        let position = glm::make_vec3(&self.position);
        let front = glm::normalize(&glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        let up = glm::vec3(0.0, 1.0, 0.0);

        *glm::look_at(&position, &(position + front), &up).as_ref()
    }
}

fn try_keycode_as_direction(keycode: sdl2::keyboard::Keycode) -> Option<MovementDirection> {
    match keycode {
        sdl2::keyboard::Keycode::W => Some(MovementDirection::Forward),
        sdl2::keyboard::Keycode::S => Some(MovementDirection::Backward),
        sdl2::keyboard::Keycode::A => Some(MovementDirection::Left),
        sdl2::keyboard::Keycode::D => Some(MovementDirection::Right),
        sdl2::keyboard::Keycode::LShift => Some(MovementDirection::Up),
        sdl2::keyboard::Keycode::LCtrl => Some(MovementDirection::Down),
        _ => None,
    }
}

pub fn process_sdl_event(camera: &mut Camera, event: sdl2::event::Event) {
    match event {
        sdl2::event::Event::KeyUp {
            keycode: Some(keycode),
            ..
        } => {
            if let Some(direction) = try_keycode_as_direction(keycode) {
                camera.set_move_direction(direction, 0.0);
            }
        }
        sdl2::event::Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => {
            if let Some(direction) = try_keycode_as_direction(keycode) {
                camera.set_move_direction(direction, 1.0);
            }
        }
        sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
            const MOUSE_SENSITIVITY: f32 = 0.1;
            let x_offset = MOUSE_SENSITIVITY * (xrel as f32);
            let y_offset = -MOUSE_SENSITIVITY * (yrel as f32);
            camera.move_view(x_offset, y_offset);
        }
        _ => {}
    }
}
