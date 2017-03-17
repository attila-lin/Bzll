use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use std::thread;

use render::render_system::RenderSystem;
use resource::resource_system::ResourcSystem;

use common;
use glutin;
use whi;
use gfx;
use gfx_window_glutin;
use state;
use specs::*;

use render::types::*;
use config;

use common::timer::*;

use std::time::{Duration, Instant};
pub struct FrameTime {
    frame_last_fps: u64,
	last_frame_time: u64,
	frame_count: i32,
	frame_rate: i32,
    delta_time: Duration,
    fixed_step: Duration,
    state: State,
}

impl FrameTime{
    pub fn new() -> Self{
        FrameTime{
            frame_last_fps: 0u64,
            last_frame_time: 0u64,
            frame_count: 0i32,
            frame_rate: 0i32,
            delta_time: Duration::new(0, 0),
            fixed_step: Duration::new(0, 16666666),
        }
    }
}

pub struct Game {
    timer: Stopwatch,
	frame_time: FrameTime,
    window: glutin::Window,
    render_system: RenderSystem,
    resource_system: ResourcSystem,
    planner: Planner<()>,
}

impl Game {
    pub fn new(config: config::Config) -> Self{
        let mut wb = whi::dxgi::window::init();

        let mut world = World::new();
        // TODO 4 -> num of core
        let mut planner = Planner::new(world, 4);

        let (window, mut device, mut factory, main_color, mut main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(wb);

        let mut render_system = RenderSystem::new(device, factory, main_color, main_depth);
        let mut rescource_system = ResourcSystem::new();
        let mut frame_time = FrameTime::new();
        Game {
            timer: Stopwatch::new(),
            frame_time: frame_time,
            window: window,
            render_system: render_system,
            resource_system: rescource_system,
            planner: planner,
        }
    }

    // TODO for different kinds of window
    // fn init_window(&mut self){

    // }

    #[allow(dead_code)]
    fn update(&mut self, elapsed_time:u64) {
        elapsed_time;
	}

    fn render(&mut self, elapsed_time:u64) {
        self.render_system.render(elapsed_time)
	}

    pub fn get_game_time() -> u64 {
        common::timer::current_time_ns()
    }

    pub fn run(&mut self) {

        while self.states.is_running() {
            self.timer.restart();
            self.advance_frame();
            self.timer.stop();
            self.frame_time.delta_time = self.timer.elapsed();
        }
    }

    fn advance_frame(&mut self) {
        for event in self.window.poll_events() {
            match event {
                glutin::Event::Closed => self.states.,
                _ => ()
            }
        }

        // let events = self.gfx_device.poll_events();
        let world = &mut self.planner.mut_world();
        // let assets = &mut self.assets;
        // let pipe = &mut self.pipe;

        // self.states.handle_events(events.as_ref(), world, assets, pipe);

        // if self.last_fixed_update.elapsed() >= self.fixed_step {
        //     self.states.fixed_update(world, assets, pipe);
        //     self.last_fixed_update += self.fixed_step;
        // }

        // self.states.update(world, assets, pipe);

        self.planner.dispatch(());
        self.planner.wait();

        if self.frame_time.last_frame_time == 0u64 {
            self.frame_time.last_frame_time = Game::get_game_time();
        }

        let frame_time = Game::get_game_time();
        let elapsed_time = frame_time - self.frame_time.last_frame_time;
        self.frame_time.last_frame_time = frame_time;

        self.update(elapsed_time);
        self.render(elapsed_time);

        self.frame_time.frame_count += 1;
        if (Game::get_game_time() - self.frame_time.frame_last_fps) >= 1000 {
            self.frame_time.frame_rate = self.frame_time.frame_count;
            self.frame_time.frame_count = 0;
            self.frame_time.frame_last_fps = Game::get_game_time();
        }
    }

    pub fn exit(&mut self){
        // tell thread to exit

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_init() {
    }
}
