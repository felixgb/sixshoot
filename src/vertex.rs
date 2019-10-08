const TRI_VERTS: usize = 3;

pub fn vertex_attrib_pointers() {
    let stride = TRI_VERTS * std::mem::size_of::<f32>();

    let location = 0;
    let offset = 0;
    vertex_attrib_pointer(location, stride, offset);

    // let location = 1;
    // let offset = offset + TRI_VERTS * std::mem::size_of::<f32>();
    // vertex_attrib_pointer(location, stride, offset);
}

fn vertex_attrib_pointer(location: usize, stride: usize, offset: usize) {
    unsafe {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            TRI_VERTS as gl::types::GLint,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

pub fn draw_arrays(vertices: &Vec<f32>) {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len() / 3) as i32);
    }
}
