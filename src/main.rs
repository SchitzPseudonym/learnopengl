use glium::Surface;

mod engine;
mod renderer;
use renderer::MyVector3;
mod math;

fn main() {
    let event_loop = engine::init_event_loop();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_inner_size(500, 500).build(&event_loop);
    
    // Example Code
    glium::implement_vertex!(MyVector3, position, tex_coords);
    //Square
    let vertices = vec![
        MyVector3 { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 1.0]}, //Top Right
        MyVector3 { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0]}, //Bottom Right
        MyVector3 { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0]}, //Bottom Left
        MyVector3 { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 1.0]}, //Top Left
    ];
    let indeces: Vec<u32> = vec![
        0, 1, 2,
        0, 3, 2
    ];
    //Texture Array
    let images = vec![
        renderer::load_png("assets/dirt.png", image::ImageFormat::Png),
    ];
    let texture_array = match glium::texture::Texture2dArray::new(&display, images) {
        Ok(data) => data,
        Err(err) => panic!("Problem glium::texture::Texture2dArray::new \n{:?}", err),
    };



    // Buffers
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indeces).unwrap();
    // Shaders
    let shader_program = engine::load_shaders(&display);

    // Event Loop
    let time_first = std::time::Instant::now();
    let mut time_last = time_first;
    match event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::RedrawRequested => {
                    let delta: u128 = time_last.elapsed().as_millis();
                    let time_running: u128 = time_first.elapsed().as_millis();
                    let mut target: glium::Frame = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    
                    let uniforms = renderer::render_loop(time_running, delta, &window, &texture_array);
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
    }) {
        Ok(_) => return,
        Err(err) => panic!("Problem event_loop.run() \n{:?}", err)
    }
}
