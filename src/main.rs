#[macro_use]
extern crate glium;
extern crate image;
extern crate time;
extern crate lazy_static;
extern crate log;
extern crate winapi;
extern crate winit;

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
    use rhi::dx12::render;
    println!("{}", render::hello());

    let mut app = game::Game::instance();
    app.create();
    app.run();
}