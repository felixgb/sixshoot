use glm::{Mat4x4, Vec3};

pub struct Camera {
    pub front: Vec3,
    pub pos: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            front: glm::vec3(0.0, 0.0, 1.0),
            pos: glm::vec3(5.0, 1.5, 5.0),
            up: glm::vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn view(&self) -> Mat4x4 {
        glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }
}

