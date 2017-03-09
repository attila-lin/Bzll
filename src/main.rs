#[macro_use]
extern crate image;
extern crate time;
extern crate lazy_static;
extern crate log;
extern crate winapi;
extern crate winit;
extern crate cgmath;

#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate rustc_serialize;
extern crate rand;
extern crate glutin;

extern crate render;

mod config;
mod common;
mod game;

mod filesystem;
mod resourcemanager;
mod camera;
mod math{
    pub mod vector;
}
mod scene;
mod rhi;
mod whi;


fn main() {
    let mut app = game::Game::instance();
    app.create();

    app.run();

    app.exit();
}