use glm::{Mat4x4, Vec3, Vec4};

pub fn translate_pos(trans: &Mat4x4, vec: &Vec3) -> Vec3 {
    let homo = to_homo(vec);
    let translated = trans * homo;
    from_homo(&translated)
}

fn to_homo(&vec: &Vec3) -> Vec4 {
    glm::vec4(vec.x, vec.y, vec.z, 1.0)
}

fn from_homo(vec: &Vec4) -> Vec3 {
    glm::vec4_to_vec3(vec)
}
