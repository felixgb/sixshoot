use glm::Vec3;

#[derive(Debug)]
pub struct AABB {
    pub left_top_front: Vec3,
    pub right_bottom_back: Vec3,
}

impl AABB {
    pub fn new(verts: &[f32]) -> AABB {
        let mut left_top_front: Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let mut right_bottom_back: Vec3 = glm::vec3(0.0, 0.0, 0.0);

        for face in verts.chunks(8) {
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

    pub fn is_in_aabb(&self, pos: Vec3) -> bool {
        self.right_bottom_back.x <= pos.x
            && self.left_top_front.x >= pos.x
            && self.right_bottom_back.z <= pos.z
            && self.left_top_front.z >= pos.z
            && self.right_bottom_back.y <= pos.y
            && self.left_top_front.y >= pos.y
    }
}

mod tests {

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

        let in_point = glm::vec3(0.0, 0.0, 0.0);
        let is_in = aabb.is_in_aabb(in_point);
        assert_eq!(is_in, true);

        let in_point = glm::vec3(1.5, 0.0, 0.0);
        let is_in = aabb.is_in_aabb(in_point);
        assert_eq!(is_in, true);

        let out_point = glm::vec3(10.0, 0.0, 0.0);
        let is_in = aabb.is_in_aabb(out_point);
        assert_eq!(is_in, false);

        let out_point = glm::vec3(10.0, 10.0, 0.0);
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
            glm::vec3(1.5, 1.5, 1.5)
        );
        assert_eq!(
            aabb.right_bottom_back,
            glm::vec3(-1.5, -1.5, -1.5)
        );
    }

}
