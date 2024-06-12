#[derive(Copy, Clone)]
pub struct Vec3 {
    pub position: [f32; 3],
}

pub fn render_loop<'a>(
    time_start: &'a std::time::Instant, 
    delta: f32,
    window: &'a winit::window::Window,
) -> impl glium::uniforms::Uniforms
{
    // let window_size = window.outer_size();
    let mut running = time_start.elapsed().as_millis() as f32;
    running = (running * 0.001).sin();
    //running = ((((running * 0.01).cos()) % 50.0) + 1.0) * 0.5;
    println!("running: {}", running);

    return glium::uniform! {
        running_time: running,
        bruh: 32.0,
        test: 2,
    };
}