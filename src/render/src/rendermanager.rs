// send command to renderengine
use image;
use gfx;

use glutin;
use super::types::*;

use filesystem;
use resourcemanager;
use camera::Camera;
use scene::scenemanager::SceneManager;

use std::io::Cursor;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};
use std::time::Duration;

pub enum RenderManagerState{
	UnInited,
	Inited,
	Exited,
}

pub struct RenderManager{
	// Since we will be used in many threads, we need to protect
	// concurrent access
	pub inner: Arc<Mutex<u8>>,
	pub sleep_time: Arc<Mutex<u64>>,
	pub state: Arc<Mutex<RenderManagerState>>,
	device: Device,
	factory: Factory,
	encoder: Encoder,
	output_color: ColorFormat,
	output_depth: OutputDepth,
	scene_manager: SceneManager,
}


impl RenderManager
{
	fn new(device:Device, factory: Factory, main_color: OutputColor, ) -> Self {
		let encoder = factory.create_command_buffer().into();

		let scene_manager = SceneManager::new();
		scene_manager.init();

		RenderManager {
			inner: Arc::new(Mutex::new(0)),
			sleep_time: Arc::new(Mutex::new(0u64)),
			state: Arc::new(Mutex::new(RenderManagerState::Inited)),
			device: device,
			factory: factory,
			encoder: encoder,
			output_color: main_color,
			output_depth: OutputDepth,
			scene_manager: scene_manager,
		}
	}

	fn on_update_begin(&self){

	}

	fn on_update_end(&self){

	}

	fn on_render_begin(&self){

	}

	fn on_render_end(&self) {

	}

	fn main_loop(&mut self) {

	}

	fn process_input(&mut self, event: &glutin::Event) {
		self.scene_manager.process_input(event);
	}

	fn start_up(&mut self) {
		self.main_loop();
	}

	fn shout_down(&mut self) {
		*self.state.lock().unwrap() = RenderManagerState::Exited;
		return;
	}

	fn update(&mut self) {
		self.on_update_begin();

		self.on_update_end();
	}

	fn render(&mut self, elapsed_time:u64) {
		match *self.state.lock().unwrap() {
			RenderManagerState::Exited => return,
			_ => {},
		}
		self.on_render_begin();
		*self.sleep_time.lock().unwrap() = (1000.0 / 60.0) as u64 - elapsed_time;
		self.on_render_end();
	}

	#[allow(dead_code)]
	fn pause(&self) {

	}

	// render engine
	fn get_renderengine(&self) {

	}
}

#[cfg(test)]
mod tests {
	use super::*;
	fn test_load_obj() {
	}
}
