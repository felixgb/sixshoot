use std::fs;
use super::model::Model;

pub fn read_map(path: &str) -> Vec<Model> {
    let texture_location = Model::cube_texture();
    let src = fs::read_to_string(path).unwrap();
    let mut models = Vec::new();

    for (x, line) in src.split("\n").enumerate() {
        for (z, char) in line.chars().enumerate() {
            if char == 'x' {
                let pos = glm::vec3(
                    (x * 4) as f32, 2.0, (z * 4) as f32
                    );
                let model = Model::test_cube_model(pos, texture_location);
                models.push(model);
            }
        }
    }

    // models.push(Model::floor_model(100, 100, 0.0));
    models
}
