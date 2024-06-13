use crate::engine;

#[derive(Copy, Clone)]
pub struct MyVector3 {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 3],
}

pub fn render_loop<'a>(
    _delta: &f32,
    _window: &'a winit::window::Window,
    texture_array: &'a glium::texture::Texture2dArray
) -> impl glium::uniforms::Uniforms + 'a
{
    // let window_size = window.outer_size();
    // let running = ((time_start.elapsed().as_millis() as f32) * 0.001).sin();
    let mut trans: glm::Mat4 = glm::Mat4::identity();
    trans = glm::rotate(&trans, engine::PI*0.5, &glm::Vec3::new(0.0, 0.0, 1.0));
    trans = glm::scale(&trans, &glm::Vec3::new(0.5, 0.5, 0.5));

    return glium::uniform! {
        texture_array: texture_array,
        transform: *trans.as_ref()
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