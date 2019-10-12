use nalgebra::*;

#[derive(Debug)]
pub struct AABB {
    pub left_top_front: Point3<f32>,
    pub right_bottom_back: Point3<f32>,
}

impl AABB {
    pub fn new(verts: &[f32]) -> AABB {
        let mut left_top_front = Point3::new(0.0, 0.0, 0.0);
        let mut right_bottom_back = Point3::new(0.0, 0.0, 0.0);

        for face in verts.chunks(3) {
            left_top_front.x = left_top_front.x.max(face[0]);
            left_top_front.y = left_top_front.y.max(face[1]);
            left_top_front.z = left_top_front.z.max(face[2]);

            right_bottom_back.x = right_bottom_back.x.min(face[0]);
            right_bottom_back.y = right_bottom_back.y.min(face[1]);
            right_bottom_back.z = right_bottom_back.z.min(face[2]);
        }
        AABB {
            left_top_front,
            right_bottom_back,
        }
    }

    pub fn is_in_aabb(&self, pos: Point3<f32>) -> bool {
        self.right_bottom_back.x <= pos.x
            && self.left_top_front.x >= pos.x
            && self.right_bottom_back.y <= pos.y
            && self.left_top_front.y >= pos.y
            && self.right_bottom_back.z <= pos.z
            && self.left_top_front.z >= pos.z
    }
}

mod tests {
    use nalgebra::{Vector3, Point3};

    #[test]
    fn test_is_in_aabb() {
        let vs = vec![
            -1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5,  1.5, -1.5,
            1.5,  1.5, -1.5,
            -1.5,  1.5, -1.5,
            -1.5, -1.5, -1.5,

            -1.5, -1.5,  1.5,
            1.5, -1.5,  1.5,
            1.5,  1.5,  1.5,
            1.5,  1.5,  1.5,
            -1.5,  1.5,  1.5,
            -1.5, -1.5,  1.5,

            -1.5,  1.5,  1.5,
            -1.5,  1.5, -1.5,
            -1.5, -1.5, -1.5,
            -1.5, -1.5, -1.5,
            -1.5, -1.5,  1.5,
            -1.5,  1.5,  1.5,

            1.5,  1.5,  1.5,
            1.5,  1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5,  1.5,
            1.5,  1.5,  1.5,

            -1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5,  1.5,
            1.5, -1.5,  1.5,
            -1.5, -1.5,  1.5,
            -1.5, -1.5, -1.5,

            -1.5,  1.5, -1.5,
            1.5,  1.5, -1.5,
            1.5,  1.5,  1.5,
            1.5,  1.5,  1.5,
            -1.5,  1.5,  1.5,
            -1.5,  1.5, -1.5,
        ];
        let aabb = super::AABB::new(&vs);

        let in_point = Point3::new(0.0, 0.0, 0.0);
        let is_in = aabb.is_in_aabb(in_point);
        assert_eq!(is_in, true);

        let in_point = Point3::new(1.5, 0.0, 0.0);
        let is_in = aabb.is_in_aabb(in_point);
        assert_eq!(is_in, true);

        let out_point = Point3::new(10.0, 0.0, 0.0);
        let is_in = aabb.is_in_aabb(out_point);
        assert_eq!(is_in, false);

        let out_point = Point3::new(10.0, 10.0, 0.0);
        let is_in = aabb.is_in_aabb(out_point);
        assert_eq!(is_in, false);
    }

    #[test]
    fn test_axis_aligned_verts() {
        let vs = vec![
            -1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5,  1.5, -1.5,
            1.5,  1.5, -1.5,
            -1.5,  1.5, -1.5,
            -1.5, -1.5, -1.5,

            -1.5, -1.5,  1.5,
            1.5, -1.5,  1.5,
            1.5,  1.5,  1.5,
            1.5,  1.5,  1.5,
            -1.5,  1.5,  1.5,
            -1.5, -1.5,  1.5,

            -1.5,  1.5,  1.5,
            -1.5,  1.5, -1.5,
            -1.5, -1.5, -1.5,
            -1.5, -1.5, -1.5,
            -1.5, -1.5,  1.5,
            -1.5,  1.5,  1.5,

            1.5,  1.5,  1.5,
            1.5,  1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5,  1.5,
            1.5,  1.5,  1.5,

            -1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5,  1.5,
            1.5, -1.5,  1.5,
            -1.5, -1.5,  1.5,
            -1.5, -1.5, -1.5,

            -1.5,  1.5, -1.5,
            1.5,  1.5, -1.5,
            1.5,  1.5,  1.5,
            1.5,  1.5,  1.5,
            -1.5,  1.5,  1.5,
            -1.5,  1.5, -1.5,
        ];
        let aabb = super::AABB::new(&vs);
        assert_eq!(
            aabb.left_top_front,
            Vector3::new(1.5, 1.5, 1.5)
        );
        assert_eq!(
            aabb.right_bottom_back,
            Vector3::new(-1.5, -1.5, -1.5)
        );
    }

}
