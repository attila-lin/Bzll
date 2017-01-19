use glium;
use image;

use std::io::Cursor;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};

#[derive(Clone)]
pub struct RenderManager {
	// Since we will be used in many threads, we need to protect
	// concurrent access
	pub inner: Arc<Mutex<u8>>
}

impl RenderManager {
	pub fn startUp(&self) {
		self.mainLoop();
	}

	pub fn shoutDown() {

	}

	pub fn instance() -> RenderManager {
		// Initialize it to a null value
		static mut SINGLETON: *const RenderManager = 0 as *const RenderManager;
		static ONCE: Once = ONCE_INIT;

		unsafe {
			ONCE.call_once(|| {
				// Make it
				let instance = RenderManager {
					inner: Arc::new(Mutex::new(0))
				};

				// Put it in the heap so it can outlive this call
				SINGLETON = mem::transmute(Box::new(instance));
			});

			// Now we give out a copy of the data that is safe to use concurrently.
			(*SINGLETON).clone()
		}
	}

	fn mainLoop(&self) {

		use glium::{DisplayBuild, Surface};
		let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

		let image = image::load(Cursor::new(&include_bytes!("../test/opengl.png")[..]),
								image::PNG).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
		let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
		let texture = glium::texture::Texture2d::new(&display, image).unwrap();

		#[derive(Copy, Clone)]
		struct Vertex {
			position: [f32; 2],
			tex_coords: [f32; 2],
		}

		implement_vertex!(Vertex, position, tex_coords);

		let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
		let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
		let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
		let shape = vec![vertex1, vertex2, vertex3];

		let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

		let vertex_shader_src = r#"
			#version 140
			in vec2 position;
			in vec2 tex_coords;
			out vec2 v_tex_coords;
			uniform mat4 matrix;
			void main() {
				v_tex_coords = tex_coords;
				gl_Position = matrix * vec4(position, 0.0, 1.0);
			}
		"#;

		let fragment_shader_src = r#"
			#version 140
			in vec2 v_tex_coords;
			out vec4 color;
			uniform sampler2D tex;
			void main() {
				color = texture(tex, v_tex_coords);
			}
		"#;

		let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let mut t: f32 = -0.5;

		loop {
			 // we update `t`
			t += 0.0002;
			if t > 0.5 {
				t = -0.5;
			}

			let uniforms = uniform! {
				matrix: [
					[ t.cos(), t.sin(), 0.0, 0.0],
					[-t.sin(), t.cos(), 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[0.0, 0.0, 0.0, 1.0f32],
				],
				tex: &texture,
			};

			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);
			//target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
			//            &Default::default()).unwrap();
			target.draw(&vertex_buffer, &indices, &program, &uniforms,
				&Default::default()).unwrap();
			target.finish().unwrap();

			for ev in display.poll_events() {
				match ev {
					glium::glutin::Event::Closed => return,
					_ => ()
				}
			}
		}
	}

}