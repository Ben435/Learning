use std::path::Path;
use image::GenericImageView;
use std::ffi::c_void;

pub unsafe fn load_texture(path: &Path) -> u32 {
    let mut texture_id = 0;

    gl::GenTextures(1, &mut texture_id);
    let img = image::open(path).expect("Texture failed to load");

    // TODO: Support more image formats. Currently just force-converts to RGB.
    let format = gl::RGB;

    let (width, height) = (img.width(), img.height());

    let data = img.into_rgb().into_raw();

    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, width as i32, height as i32,
        0, format, gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    texture_id
}
