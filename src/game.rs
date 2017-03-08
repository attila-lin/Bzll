use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use std::thread;
use rand::{self, Rng};

use render::rendermanager::RenderManager;

use common;

use winit;
use whi;
use gfx;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

// TODO move to config
const QUAD_VERTICES: [Vertex; 4] = [Vertex { position: [-0.5, 0.5] },
    Vertex { position: [-0.5, -0.5] },
    Vertex { position: [0.5, -0.5] },
    Vertex { position: [0.5, 0.5] }];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "a_Position",
    }

    // color format: 0xRRGGBBAA
    vertex Instance {
        translate: [f32; 2] = "a_Translate",
        color: u32 = "a_Color",
    }

    constant Locals {
        scale: f32 = "u_Scale",
    }

    pipeline pipe {
        vertex: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        scale: gfx::Global<f32> = "u_Scale",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

fn fill_instances(instances: &mut [Instance], instances_per_length: u32, size: f32) {
    let gap = 0.4 / (instances_per_length + 1) as f32;
    println!("gap: {}", gap);

    let begin = -1. + gap + (size / 2.);
    let mut translate = [begin, begin];
    let mut rng = rand::StdRng::new().unwrap();

    let length = instances_per_length as usize;
    for x in 0..length {
        for y in 0..length {
            let i = x * length + y;
            instances[i] = Instance {
                translate: translate,
                color: rng.next_u32(),
            };
            translate[1] += size + gap;
        }
        translate[1] = begin;
        translate[0] += size + gap;
    }
}

const MAX_INSTANCE_COUNT: usize = 2048;

struct App<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    upload: gfx::handle::Buffer<R, Instance>,
    uploading: bool, // TODO: not needed if we have the encoder everywhere
}

impl<R> App<R>
    where R: gfx::Resources
{
    fn new<F>(factory: &mut F,
              color_format: gfx::handle::RenderTargetView<R,
                  (gfx::format::R8_G8_B8_A8,
                   gfx::format::Unorm)>)
              -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>
    {

        let pso = factory.create_pipeline_simple(include_bytes!("../artist/shader/instancing_150.glslv"),
                                                 include_bytes!("../artist/shader/instancing_150.glslf"),
                                                 pipe::new())
            .unwrap();

        let instances_per_length: u32 = 32;
        println!("{} instances per length", instances_per_length);
        let instance_count = instances_per_length * instances_per_length;
        println!("{} instances", instance_count);
        assert!(instance_count as usize <= MAX_INSTANCE_COUNT);
        let size = 1.6 / instances_per_length as f32;
        println!("size: {}", size);

        let upload = factory.create_upload_buffer(instance_count as usize).unwrap();
        {
            let mut writer = factory.write_mapping(&upload).unwrap();
            fill_instances(&mut writer, instances_per_length, size);
        }

        let instances = factory.create_buffer(instance_count as usize,
                                              gfx::buffer::Role::Vertex,
                                              gfx::memory::Usage::Data,
                                              gfx::TRANSFER_DST)
            .unwrap();


        let (quad_vertices, mut slice) =
            factory.create_vertex_buffer_with_slice(&QUAD_VERTICES, &QUAD_INDICES[..]);
        slice.instances = Some((instance_count, 0));
        let locals = Locals { scale: size };

        App {
            pso: pso,
            data: pipe::Data {
                vertex: quad_vertices,
                instance: instances,
                scale: size,
                locals: factory.create_buffer_immutable(&[locals],
                                                        gfx::buffer::Role::Constant,
                                                        gfx::Bind::empty())
                    .unwrap(),
                out: color_format,
            },
            slice: slice,
            upload: upload,
            uploading: true,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        if self.uploading {
            encoder.copy_buffer(&self.upload, &self.data.instance, 0, 0, self.upload.len())
                .unwrap();
            self.uploading = false;
        }

        encoder.clear(&self.data.out, [0.1, 0.2, 0.3, 1.0]);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}

#[derive(Clone)]
pub struct Game {
	pub inner: Arc<Mutex<u8>>,
	frame_last_fps: u64,
	last_frame_time: u64,
	frame_count: i32,
	frame_rate: i32,
}


impl Game {
    fn new() -> Self{
        Game {
            inner: Arc::new(Mutex::new(0)),
            frame_last_fps: 0u64,
            last_frame_time: 0u64,
            frame_count: 0,
            frame_rate: 0,
        }
    }

	pub fn instance() -> Box<GInterface> {
        // Initialize it to a null value
        static mut SINGLETON: *const Game = 0 as *const Game;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                // Put it in the heap so it can outlive this call
                SINGLETON = mem::transmute(Box::new(Game::new()));
            });

            // Now we give out a copy of the data that is safe to use concurrently.
            Box::new((*SINGLETON).clone())
        }
    }

    fn init(&mut self) {
        self.init_window();

        // self.init_render_enginer();
    }
    // TODO for different platform
    fn init_window(&mut self){
        let mut wb = whi::dxgi::window::init();

        let (window, mut device, mut factory, main_color, mut _main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(wb);
        let mut app = App::new(&mut factory, main_color);

        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        // let (mut window, device, mut factory, main_color) = wb.unwrap();
        'main: loop {
            for event in window.poll_events() {
                match event {
                    winit::Event::Closed => break 'main,
                    _ => ()
                }
            }
            // draw a frame
            app.render(&mut encoder);
            encoder.flush(&mut device);
            window.swap_buffers().unwrap();
            device.cleanup();
        }
    }

    // TODO for different platform
    fn init_render_enginer(&self){
        // render create
        let render_thread = thread::spawn(move || {
            RenderManager::instance().start_up();
        });

        render_thread.join().unwrap();
    }

    fn create_command_objects(){

    }

    #[allow(dead_code)]
    fn update(elapsed_time:u64) {
        elapsed_time;
	}

    fn render(elapsed_time:u64) {
        RenderManager::instance().render(elapsed_time)
	}

    pub fn get_game_time() -> u64 {
        common::timer::current_time_ns()
    }
}


pub trait GInterface {
    fn create(&mut self);
    fn run(&mut self);
    fn exit(&mut self);
}

impl GInterface for Game {
    fn create(&mut self) {

        self.init();
    }

    // main loop
    fn run(&mut self) {
        if self.last_frame_time == 0u64 {
            self.last_frame_time = Game::get_game_time();
        }

        let frame_time = Game::get_game_time();
        let elapsed_time = frame_time - self.last_frame_time;
        self.last_frame_time = frame_time;

        Game::update(elapsed_time);
        Game::render(elapsed_time);

        self.frame_count += 1;
        if (Game::get_game_time() - self.frame_last_fps) >= 1000 {
            self.frame_rate = self.frame_count;
            self.frame_count = 0;
            self.frame_last_fps = Game::get_game_time();
        }
    }

    fn exit(&mut self){

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_load_obj() {
    }
}
