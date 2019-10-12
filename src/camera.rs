use nalgebra::*;

pub struct Camera {
    pub front: Vector3<f32>,
    pub pos: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            front: Vector3::new(0.0, 0.0, 0.0),
            pos: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::y(),
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        let p = Point3::from(self.pos);
        let v = Point3::from(self.pos + self.front);
        Isometry3::look_at_rh(&p, &v, &self.up).to_homogeneous()
    }
}

