#[macro_use]
extern crate glium;
mod audio;
// extern crate time;
// use time::PreciseTime;

fn main() {
    audio::run().unwrap_or_else(|x| eprintln!("{:?}", x));
    use glium::glutin::WindowEvent::*;
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let hidpi_factor = display.gl_window().window().get_hidpi_factor();
    println!("{}", hidpi_factor);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex {
            position: [-1., -1.],
        },
        Vertex { position: [1., 1.] },
        Vertex {
            position: [-1., 1.],
        },
        Vertex {
            position: [-1., -1.],
        },
        Vertex {
            position: [1., -1.],
        },
        Vertex { position: [1., 1.] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices =
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = include_str!("fragment.glsl");

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None,
    )
    .unwrap();

    const BUF_LEN: usize = 1024;
    let gpu_buffer =
        glium::uniforms::UniformBuffer::<[[f32; 4]; BUF_LEN/4]>::empty_persistent(
            &display,
        )
        .unwrap();
    let mut buffer = [[0.0f32; 4]; BUF_LEN / 4];
    let mut mouse = [0.0f32; 2];

    let mut closed = false;
    while !closed {
        // let start = PreciseTime::now();

        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        for i in 0..BUF_LEN {
            buffer[i >> 2][i & 3] = f32::sin((i as f32) / 32.0) / 2.0;
        }
        gpu_buffer.write(&buffer);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! {
                    windowSize: [width as f32, height as f32],
                    Buffer: &gpu_buffer,
                    mouse: mouse,
                },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                CloseRequested => closed = true,
                CursorMoved { position, .. } => {
                    println!(
                        "x:{:6}, y:{:6} w:{:6} h:{:6}",
                        (position.x * hidpi_factor) as i32,
                        (position.y * hidpi_factor) as i32,
                        width,
                        height
                    );
                    mouse = [
                        hidpi_factor as f32 * position.x as f32 / width as f32,
                        1f32 - hidpi_factor as f32 * position.y as f32
                            / height as f32,
                    ];
                }
                _ => (),
            },
            _ => (),
        });

        // let end = PreciseTime::now();
        /* println!(
         * "Current framerate: {}hz",
         * 1f32 / (start.to(end).num_microseconds().unwrap() as f32
         * / 1_000_000f32)
         * );
         */
    }
}
