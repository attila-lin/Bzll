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

extern crate render;
extern crate scene;

mod config;
mod common;
mod game;

mod filesystem;
mod resourcemanager;
mod math{
    pub mod vector;
}
mod rhi;
mod whi;


fn main() {
    let mut app = game::Game::new();

    app.run();

    app.exit();
}