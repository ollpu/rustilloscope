#[macro_use]
extern crate glium;
extern crate time;
use time::PreciseTime;

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
    let fragment_shader_src = r#"
        #version 140
        #define SIZE 1024

        uniform vec2 windowSize;
        uniform vec2 mouse;
        out vec4 color;

        float distLine(vec2 line, vec2 point) {
            float s = dot(line, point) / dot(line, line);
            vec2 x = point-line*s;
            return length(x);
        }

        layout(packed) uniform Buffer {
            vec4 array[SIZE/4];
        };

        void main() {
            int idx = int(gl_FragCoord.x/windowSize.x*float(SIZE));
            float val = array[idx >> 2][idx & 3]/2.0+0.25;
            float nval = (idx+1==SIZE)
                ? -100.0
                : array[idx+1 >> 2][idx+1 & 3]/2.0+0.25;

            vec2 diffline = vec2(1, windowSize.y*(nval - val));
            vec2 point = vec2(0, gl_FragCoord.y - val*windowSize.y);

            float dist = distLine(diffline, point);

            vec2 mousepos = vec2(
                sqrt(2)*windowSize.x * mouse.x,
                windowSize.y * (1-sqrt(2)*mouse.y)
            );
            if (length(mousepos - gl_FragCoord.xy) < 10.0) {
                color = vec4(1., 1., 1., 1.);
                return;
            }
            if (dist < 3) {
                color = vec4(0.2, 1., 0.2, 1.);
            } else {
                color = vec4(0., 0.2, 0., 1.);
            }
        }
    "#;

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
        let start = PreciseTime::now();

        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        for i in 0..BUF_LEN {
            buffer[i >> 2][i & 3] = f32::sin((i as f32) / 32.0) / 2.0 + 0.5;
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
                    println!("{:?}, {} {}", position, width, height);
                    mouse = [
                        position.x as f32 / width as f32,
                        position.y as f32 / height as f32,
                    ];
                }
                _ => (),
            },
            _ => (),
        });

        let end = PreciseTime::now();
        /* println!(
         * "Current framerate: {}hz",
         * 1f32 / (start.to(end).num_microseconds().unwrap() as f32
         * / 1_000_000f32)
         * );
         */
    }
}
