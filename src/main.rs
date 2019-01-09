#[macro_use]
extern crate glium;

fn main() {
    use glium::glutin::WindowEvent::*;
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];
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
    let fragment_shader_src = r#"
        #version 140

        uniform vec2 windowSize;
        out vec4 color;

        uniform Buffer {
            float kana[512];
        };

        void main() {
            float val = kana[int(gl_FragCoord.x) % 128];
            color = vec4(val, val, val, 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None,
    )
    .unwrap();

    const BUF_LEN: usize = 512;
    let gpu_buffer =
        glium::uniforms::UniformBuffer::<[f32; BUF_LEN]>::empty_persistent(
            &display,
        )
        .unwrap();
    let mut buffer: [f32; BUF_LEN] = [0.0; BUF_LEN];

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        for i in 0..BUF_LEN {
            buffer[i] = f32::sin((i as f32) / 4.0) / 2.0 + 0.5;
        }
        gpu_buffer.write(&buffer);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                // &glium::uniforms::EmptyUniforms,
                &uniform! {
                    windowSize: [width as f32, height as f32],
                    Buffer: &gpu_buffer,
                },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                CloseRequested => closed = true,
                CursorMoved { position, .. } => println!("{:?}", position),
                _ => (),
            },
            _ => (),
        });
    }
}
