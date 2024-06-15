use crate::{engine::{self, PI}, math::{self, Mat4, Vec3}};

#[derive(Copy, Clone)]
pub struct MyVector3 {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub fn render_loop<'a>(
    time_running: u128,
    _delta: u128,
    window: &'a winit::window::Window,
    texture_array: &'a glium::texture::Texture2dArray
) -> impl glium::uniforms::Uniforms + 'a
{
    let window_size = window.outer_size();
    let mut transform = Mat4::new(1.0);

    // glm::mat4 proj = glm::perspective(glm::radians(45.0f), (float)width/(float)height, 0.1f, 100.0f);
    transform = transform * Mat4::transform((time_running as f32 / 1000.0) % (PI*2.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.5, -0.5, 0.0));
    
    let orthographic = Mat4::orthographic(-1.0, 1.0, 1.0, -1.0, 0.1, 1000.0);
    let perspective = Mat4::perspective(PI*0.5, (window_size.width/window_size.height) as f32, 0.1, 1000.0);

    return glium::uniform! {
        texture_array: texture_array,
        // transform: *transform.as_ref()
        transform: transform.data,
        orthographic: orthographic.data,
        perspective: perspective.data
    };
}

pub fn load_png(filename: &str, format: image::ImageFormat) -> glium::texture::RawImage2d<u8>
{
    let read_data =
    match std::fs::read(filename)
    {
        Ok(data) => data,
        Err(err) => panic!("Problem std::fs::read \"{}\":\n{:?}", filename, err),
    };
    
    let image_data =
    match image::load(std::io::Cursor::new(read_data), format)
    {
        Ok(data) => data.to_rgba8(),
        Err(err) => panic!("Problem image::load \"{}\":\n{:?}", filename, err),
    };
    
    let image_dimensions = image_data.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image_data.into_raw(), image_dimensions);
    //upload the image to the GPU
    return image;
}