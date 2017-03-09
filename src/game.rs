use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use std::thread;

use render::rendermanager::RenderManager;

use common;
use glutin;
use winit;
use whi;
use gfx;
use gfx_window_glutin;

use render::types::*;

#[derive(Clone)]
pub struct Game {
	pub inner: Arc<Mutex<u8>>,
	frame_last_fps: u64,
	last_frame_time: u64,
	frame_count: i32,
	frame_rate: i32,
    window: glutin::Window,
    renderManager: RenderManager,
}

impl Game {
    fn new() -> Self{
        let mut wb = whi::dxgi::window::init();

        let (window, mut device, mut factory, main_color, mut _main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(wb);

        let renderManager = RenderManager::new(device, factory, main_color);

        Game {
            inner: Arc::new(Mutex::new(0)),
            frame_last_fps: 0u64,
            last_frame_time: 0u64,
            frame_count: 0,
            frame_rate: 0,
            window:window,
            renderManager:renderManager,
        }
    }

    fn init_render_enginer(&mut self){

        // render create
        let render_thread = thread::spawn(move || {
            self.renderManager.start_up();
        });

        render_thread.join().unwrap();
    }

    fn init(&mut self) {

    }

    // TODO for different kinds of window
    fn init_window(&mut self){

    }

    // TODO for different platform


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


    fn create(&mut self) {
        self.init();
    }

    fn run(&mut self) {
        'main: loop {
            for event in self.window.poll_events() {
                match event {
                    winit::Event::Closed => break 'main,
                    _ => ()
                }
            }

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
    }

    fn exit(&mut self){
        // tell thread to exit

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_init() {
    }
}
