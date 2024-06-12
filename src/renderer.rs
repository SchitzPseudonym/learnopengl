use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub position: [f32; 3],
    pub texcoords: [f32; 3],
}

pub fn render_loop
(
    time_last: &std::time::Instant,
    window: &winit::window::Window,
    display: &glium::Display<glium::glutin::surface::WindowSurface>,
    vertex_buffer: &glium::VertexBuffer<Vec3>,
    index_buffer: &glium::IndexBuffer<u32>,
    shader_program: &glium::Program,
) -> std::time::Instant
{
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    
    // Calculate Frame
    let delta = time_last.elapsed().as_nanos() as f32 * 0.0000001;

    let window_size = window.outer_size();

    let uniforms = glium::uniform! {
        
    };
    
    // Draw Frame
    target.draw(vertex_buffer, index_buffer, shader_program, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    // Return new time_last
    return std::time::Instant::now()
}