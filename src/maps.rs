use std::fs;
use super::model::Model;

pub fn read_map(path: &str) -> Vec<Model> {
    let src = fs::read_to_string(path).unwrap();
    let mut models = Vec::new();

    for (x, line) in src.split("\n").enumerate() {
        for (z, char) in line.chars().enumerate() {
            if char == 'x' {
                let pos = glm::vec3(
                    (x * 3) as f32, 1.5, (z * 3) as f32
                    );
                let model = Model::test_cube_model(pos);
                models.push(model);
            }
        }
    }

    models.push(Model::floor_model(100, 100, 0.0));
    models.push(Model::floor_model(100, 100, 3.0));
    models
}
