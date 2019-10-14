use glm::{Mat4x4, Vec3};
use super::shader_program::*;
use super::uniform::{get_uniform_location, Uniform};

pub struct MVPUniforms {
    view_loc: Uniform,
    model_loc: Uniform,
    projection_loc: Uniform,
}

impl MVPUniforms {
    pub fn for_program(program: &Program) -> MVPUniforms {
        let projection_loc = get_uniform_location(program.id, "projection").unwrap();
        let view_loc = get_uniform_location(program.id, "view").unwrap();
        let model_loc = get_uniform_location(program.id, "model").unwrap();
        MVPUniforms { model_loc, view_loc, projection_loc, }
    }

    pub fn set_vp(&self, view: &Mat4x4, projection: &Mat4x4) {
        self.view_loc.set_uniform_matrix4fv(view);
        self.projection_loc.set_uniform_matrix4fv(projection);
    }

    pub fn set_m(&self, model: &Mat4x4) {
        self.model_loc.set_uniform_matrix4fv(model);
    }
}

pub struct LightUniforms {
    color_loc: Uniform,
    pos_loc: Uniform,
    object_color_loc: Uniform,
}

impl LightUniforms {
    pub fn for_program(program: &Program) -> LightUniforms {
        let color_loc = get_uniform_location(program.id, "light_color").unwrap();
        let object_color_loc = get_uniform_location(program.id, "object_color").unwrap();
        let pos_loc = get_uniform_location(program.id, "light_pos").unwrap();
        LightUniforms { color_loc, object_color_loc, pos_loc, }
    }

    pub fn set_light(&self, pos: &Vec3, color: &Vec3) {
        self.color_loc.set_uniform_vec3(color);
        self.pos_loc.set_uniform_vec3(pos);
    }

    pub fn set_object_color(&self, color: &Vec3) {
        self.object_color_loc.set_uniform_vec3(color);
    }
}

pub struct LightProgram {
    pub program: Program,
    pub mvp: MVPUniforms,
}

impl LightProgram {
    pub fn from_shaders(vert: &Shader, frag: &Shader) -> LightProgram {
        let program = Program::from_shaders(vert, frag).unwrap();
        let mvp = MVPUniforms::for_program(&program);
        LightProgram { program, mvp }
    }
}

pub struct ModelProgram {
    pub program: Program,
    pub mvp: MVPUniforms,
    pub lights: LightUniforms,
}

impl ModelProgram {
    pub fn from_shaders(vert: &Shader, frag: &Shader) -> ModelProgram {
        let program = Program::from_shaders(vert, frag).unwrap();
        let mvp = MVPUniforms::for_program(&program);
        let lights = LightUniforms::for_program(&program);
        ModelProgram { program, mvp, lights }
    }
}
