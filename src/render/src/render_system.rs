// send command to renderengine
use gfx;
use glutin;

use types::*;

use camera::Camera;

use std::io::Cursor;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};
use std::time::Duration;

pub enum RenderSystemState{
	UnInited,
	Inited,
	Exited,
}

pub struct RenderSystem{
	// Since we will be used in many threads, we need to protect
	// concurrent access
	pub inner: Arc<Mutex<u8>>,
	pub sleep_time: Arc<Mutex<u64>>,
	pub state: Arc<Mutex<RenderSystemState>>,
	device: Device,
	factory: Factory,
	encoder: Encoder,
	output_color: OutputColor,
	output_depth: OutputDepth,
}


impl RenderSystem
{
	pub fn new(device:Device, mut factory: Factory, main_color: OutputColor, main_depth: OutputDepth ) -> Self {
		let encoder = factory.create_command_buffer().into();

		RenderSystem {
			inner: Arc::new(Mutex::new(0)),
			sleep_time: Arc::new(Mutex::new(0u64)),
			state: Arc::new(Mutex::new(RenderSystemState::Inited)),
			device: device,
			factory: factory,
			encoder: encoder,
			output_color: main_color,
			output_depth: main_depth,
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
		// self.scene_system.process_input(event);
	}

	pub fn start_up(&mut self) {
		self.main_loop();
	}

	fn shout_down(&mut self) {
		*self.state.lock().unwrap() = RenderSystemState::Exited;
		return;
	}

	fn update(&mut self) {
		self.on_update_begin();

		self.on_update_end();
	}

	pub fn render(&mut self, elapsed_time:u64) {
		match *self.state.lock().unwrap() {
			RenderSystemState::Exited => return,
			_ => {},
		}
		self.on_render_begin();

		// *self.sleep_time.lock().unwrap() = (1000.0 / 60.0) as u64 - elapsed_time;

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
