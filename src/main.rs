#[macro_use]
extern crate time;
extern crate lazy_static;
extern crate log;
extern crate winapi;
extern crate winit;
extern crate cgmath;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate rustc_serialize;
extern crate glutin;

//#[macro_use]
//extern crate serde_derive;
extern crate toml;

extern crate render;
extern crate scene;
extern crate resource;

mod config;
mod common;
mod game;

mod filesystem;
mod math{
    pub mod vector;
}
mod rhi;
mod whi;


fn main() {
    let config = config::Config::get_config();
    let mut app = game::Game::new(config);

    app.run();

    app.exit();
}