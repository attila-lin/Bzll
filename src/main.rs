#[macro_use]
extern crate glium;
extern crate image;
extern crate time;
extern crate lazy_static;
extern crate log;
extern crate winapi;
extern crate winit;
extern crate cgmath;
extern crate gfx;
extern crate gfx_window_glutin;

mod config;
mod common;
mod game;
mod render;
mod filesystem;
mod resourcemanager;
mod camera;
mod math{
    pub mod vector;
}
mod scene;
mod rhi;
mod whi{
    pub mod window;
    pub mod dxgi;
    pub mod glutin;
}

fn main() {
    let mut app = game::Game::instance();
    app.create();
    app.run();

    app.exit();
}