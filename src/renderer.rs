use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub position: [f32; 3],
}

pub fn render_loop
(
    time_start: &std::time::Instant, 
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

    // let window_size = window.outer_size();
    let mut running = time_start.elapsed().as_millis() as f32;
    running = (running * 0.001).sin();
    //running = ((((running * 0.01).cos()) % 50.0) + 1.0) * 0.5;
    println!("running: {}", running);

    let uniforms = glium::uniform! {
        running_time: running,
    };
    
    // Draw Frame
    target.draw(vertex_buffer, index_buffer, shader_program, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    // Return new time_last
    return std::time::Instant::now()
}