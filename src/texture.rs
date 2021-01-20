use image::EncodableLayout;

pub fn prepare_textures() -> gl::types::GLuint {
    let mut texture_id: gl::types::GLuint = 0;
    let image = image::open("assets/container.jpg").unwrap().to_rgb8();
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as gl::types::GLint,
            image.width() as gl::types::GLint,
            image.height() as gl::types::GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            image.as_bytes().as_ptr() as *const gl::types::GLvoid
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    texture_id
}

