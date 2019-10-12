use super::camera;
use nalgebra::Vector3;
use glfw::{Key, Action};
use std::collections::HashSet;

const SENSITIVITY: f32 = 0.5;

pub struct Controls<'a> {
    pub camera: &'a mut camera::Camera,

    last_x: f32,
    last_y: f32,

    yaw: f32,
    pitch: f32,

    pressed: HashSet<Key>,
}

impl<'a> Controls<'a> {
    pub fn new(camera: &'a mut camera::Camera) -> Controls {
        Controls {
            camera,
            last_x: 1920.0 / 2.0,
            last_y: 1080.0 / 2.0,
            yaw: 90.0,
            pitch: 0.0,
            pressed: HashSet::new(),
        }
    }

    pub fn mouse_callback(&mut self, x: f32, y: f32) {
        let x_offset = (x - self.last_x) as f32;
        let y_offset = (self.last_y - y) as f32;

        self.last_x = x;
        self.last_y = y;

        self.yaw += x_offset * SENSITIVITY;
        self.pitch += y_offset * SENSITIVITY;
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
            );
        self.camera.front = front.normalize()
    }

    pub fn update(&mut self, delta_millis: f32) {
        let forward = self.pressed.contains(&Key::W);
        let backward = self.pressed.contains(&Key::S);
        let left = self.pressed.contains(&Key::A);
        let right = self.pressed.contains(&Key::D);
        let camera_speed = 0.005 * delta_millis;

        if forward {
            self.camera.pos += camera_speed * self.camera.front;
        } else if backward {
            self.camera.pos -= camera_speed * self.camera.front;
        }
        if right {
            self.camera.pos += self.camera.front.cross(&self.camera.up).normalize() * camera_speed;
        } else if left {
            self.camera.pos -= self.camera.front.cross(&self.camera.up).normalize() * camera_speed;
        }
    }

    pub fn key_move_callback(&mut self, key: Key, action: Action) {
        match action {
            Action::Press => {
                self.pressed.insert(key);
            }
            Action::Release => {
                self.pressed.remove(&key);
            }
            _ => { }
        };
    }
}
