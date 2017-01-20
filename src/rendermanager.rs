use glium;
use image;

use mainwnd::MainWnd;
use filesystem;

use std::io::Cursor;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};

#[derive(Clone)]
pub struct RenderManager {
	// Since we will be used in many threads, we need to protect
	// concurrent access
	pub inner: Arc<Mutex<u8>>,
	pub _sleepTime: u64,
}

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	normal: [f32; 3],
	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
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
					inner: Arc::new(Mutex::new(0)),
					_sleepTime: 0u64,
				};

				// Put it in the heap so it can outlive this call
				SINGLETON = mem::transmute(Box::new(instance));
			});

			// Now we give out a copy of the data that is safe to use concurrently.
			(*SINGLETON).clone()
		}
	}

	pub fn render(&mut self, elapsedTime:u64)
	{
		self._sleepTime = (1000.0 / 60.0) as u64 - elapsedTime;
	}

	fn pause()
	{

	}

	fn mainLoop(&self) {

		use glium::{DisplayBuild, Surface};
		let display = glium::glutin::WindowBuilder::new()
						.with_depth_buffer(24)
						.build_glium().unwrap();

		let shape = glium::vertex::VertexBuffer::new(&display, &[
				Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
				Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
				Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
				Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
			]).unwrap();


		let image = image::load(Cursor::new(&include_bytes!("../test/tuto-14-diffuse.jpg")[..]),
								image::JPEG).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
		let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
		let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

		let image = image::load(Cursor::new(&include_bytes!("../test/tuto-14-normal.png")[..]),
								image::PNG).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
		let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
		let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();


		let vertex_shader_src = filesystem::open("./test/vertex_shader");
		let vertex_shader_src_slice: &str = &vertex_shader_src;
		let fragment_shader_src = filesystem::open("./test/fragment_shader");
		let fragment_shader_src_slice: &str = &fragment_shader_src;

		let program = glium::Program::from_source(&display, vertex_shader_src_slice, fragment_shader_src_slice,
												  None).unwrap();

		let mut t:f32 = -0.5;

		let view = view_matrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

		loop {
			let mut target = display.draw();
			target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

			t += 0.001;
			if t > 0.5 {
				t = -0.5;
			}
			let model = [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[t, 0.0, 0.0, 1.0f32]
			];

			let perspective = {
				let (width, height) = target.get_dimensions();
				let aspect_ratio = height as f32 / width as f32;

				let fov: f32 = 3.141592 / 3.0;
				let zfar = 1024.0;
				let znear = 0.1;

				let f = 1.0 / (fov / 2.0).tan();

				[
					[f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
					[         0.0         ,     f ,              0.0              ,   0.0],
					[         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
					[         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
				]
			};

			let light = [1.4, 0.4, 0.7f32];

			let params = glium::DrawParameters {
				depth: glium::Depth {
					test: glium::draw_parameters::DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				.. Default::default()
			};

			target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
						&uniform! { model: model, view: view, perspective: perspective,
									u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &normal_map },
						&params).unwrap();
			target.finish().unwrap();

			use std::time::Duration;
			use std::thread;
			thread::sleep(Duration::from_millis(self._sleepTime));

			for ev in display.poll_events() {
				match ev {
					glium::glutin::Event::Closed => return,
					glium::glutin::Event::KeyboardInput(elementState, u8, keyCode) => KeyboardInput(elementState, keyCode),
					_ => ()
				}
			}
		}
	}

}

use glium::glutin::{ElementState, VirtualKeyCode};
pub fn KeyboardInput(elementState:ElementState, keyCode:Option<VirtualKeyCode>)
{
	if elementState == ElementState::Pressed{
		print!("down{:?}", keyCode);
	}
	else {
		print!("up{:?}", keyCode);
	}
}