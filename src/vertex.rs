const TRI_VERTS: usize = 3;

pub fn vertex_attrib_pointers() {
    let stride = (2 * TRI_VERTS + 2) * std::mem::size_of::<f32>();

    let location = 0;
    let offset = 0;
    vertex_attrib_pointer(location, TRI_VERTS as gl::types::GLint, stride, offset);

    let location = 1;
    let offset = offset + TRI_VERTS * std::mem::size_of::<f32>();
    vertex_attrib_pointer(location, TRI_VERTS as gl::types::GLint, stride, offset);

    let location = 2;
    let offset = offset + TRI_VERTS  * std::mem::size_of::<f32>();
    vertex_attrib_pointer(location, 2, stride, offset);
}

pub fn vertex_attrib_pointer(location: usize, size: gl::types::GLint, stride: usize, offset: usize) {
    unsafe {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            size,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

pub fn draw_arrays(num_faces: usize) {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, num_faces as i32);
    }
}
