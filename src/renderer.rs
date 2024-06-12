#[derive(Copy, Clone)]
pub struct Vec3 {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 3],
}

pub fn render_loop<'a>(
    time_start: &'a std::time::Instant, 
    delta: f32,
    window: &'a winit::window::Window,
    texture: &'a glium::texture::Texture2d
) -> impl glium::uniforms::Uniforms + 'a
{
    // let window_size = window.outer_size();
    // let running = ((time_start.elapsed().as_millis() as f32) * 0.001).sin();

    return glium::uniform! {
        our_texture: texture,
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
