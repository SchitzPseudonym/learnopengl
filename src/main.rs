#![allow(unused_variables, unused_mut)]

mod renderer;
use glium::Surface;
use renderer::Vec3;

fn main() {
    // Boilerplate
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    glium::implement_vertex!(Vec3, position, tex_coords, color);

    // Example Triangle
    let vertices = vec![
        Vec3 { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 1.0], color: [1.0, 0.0, 0.0]}, //Top Right
        Vec3 { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0], color: [0.0, 1.0, 0.0]}, //Bottom Right
        Vec3 { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0], color: [0.0, 0.0, 1.0]}, //Bottom Left
        Vec3 { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 1.0], color: [1.0, 1.0, 0.0]}, //Top Left
    ];

    let indeces: Vec<u32> = vec![
        0, 1, 2,
        0, 3, 2
    ];
    

    let images = vec![
        renderer::load_png("assets/Salsa Water.png", image::ImageFormat::Png),
        renderer::load_png("assets/Fuck it we ball.png", image::ImageFormat::Png)
    ];

    let texture_array = match glium::texture::Texture2dArray::new(&display, images) {
        Ok(data) => data,
        Err(err) => panic!("Problem glium::texture::Texture2dArray::new \n{:?}", err),
    };

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
                    let delta: f32 = time_last.elapsed().as_nanos() as f32 * 0.0000001;
                    let mut target: glium::Frame = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    
                    let uniforms = renderer::render_loop(&time_start, delta, &window, &texture_array);
                    target.draw(&vertex_buffer, &index_buffer, &shader_program, &uniforms, &Default::default()).unwrap();
                    
                    target.finish().unwrap();
                    time_last = std::time::Instant::now();
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