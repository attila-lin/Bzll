use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use std::thread;

use rendermanager::RenderManager;

use common;

use winit;
use whi;

#[derive(Clone)]
pub struct Game {
	pub inner: Arc<Mutex<u8>>,
	frame_last_fps: u64,
	last_frame_time: u64,
	frame_count: i32,
	frame_rate: i32,
}

pub trait GInterface {
    fn create(&mut self);
    fn run(&mut self);
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

        println!("render");
        // self.init_render_enginer();
    }

    fn init_window(&mut self){

        let mut win = whi::dxgi::window::init().unwrap();

        // let (mut window, device, mut factory, main_color) = wb.unwrap();
        for event in win.inner.wait_events() {
            match event {
                winit::Event::Closed => break,
                _ => ()
            }
        }
    }

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

impl GInterface for Game {
    fn create(&mut self) {
        // init window
        self.init();


    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_load_obj() {
    }
}
