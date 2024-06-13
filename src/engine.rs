pub fn init_event_loop() -> winit::event_loop::EventLoop<()>
{
    match winit::event_loop::EventLoopBuilder::new().build() { Ok(el) => return el, Err(err) => panic!("Problem glium::texture::Texture2dArray::new \n{:?}", err), };
}

pub fn load_shaders(display: &glium::Display<glium::glutin::surface::WindowSurface>) -> glium::Program
{
    let vertex_shader_src: String = match std::fs::read_to_string("shaders/vertex_shader.vert") { Ok(result) => result, Err(err) => panic!("Problem std::fs::read_to_string \n{:?}", err), };
    let fragment_shader_src: String = match std::fs::read_to_string("shaders/fragment_shader.frag") { Ok(result) => result, Err(err) => panic!("Problem std::fs::read_to_string \n{:?}", err), };
    match glium::Program::from_source(display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None) { Ok(result) => return result, Err(err) => panic!("Problem glium::Program::from_source \n{:?}", err), }
}