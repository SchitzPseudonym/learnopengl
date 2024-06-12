#![allow(unused_variables, unused_mut)]

mod renderer;
use renderer::Vec3;

fn main() {
    // Boilerplate
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    glium::implement_vertex!(Vec3, position);

    // Example Triangle
    let vertices = vec![
        Vec3 { position: [0.5, -0.5, 0.0]}, // Bottom Right
        Vec3 { position: [-0.5,  -0.5, 0.0]}, // Bottom Left
        Vec3 { position: [0.0, 0.5, 0.0]}, // Top
    ];
    let indeces: Vec<u32> = vec![
        0, 1, 2,
    ];

    // Buffers
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indeces).unwrap();
    // Shaders
    let vertex_shader_src: &str = &std::fs::read_to_string("shaders/vertex_shader.vert").expect("failed to read vertex shader");
    let fragment_shader_src: &str = &std::fs::read_to_string("shaders/fragment_shader.frag").expect("failed to read fragment shader");
    let shader_program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let time_start = std::time::Instant::now();
    let mut time_last = time_start;
    // Event Loop
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::RedrawRequested => {
                    // Reset Frame
                    time_last = renderer::render_loop(&time_start, &time_last, &window, &display, &vertex_buffer, &index_buffer, &shader_program);
                }
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}