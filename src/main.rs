#[macro_use]
extern crate glium;
extern crate image;
extern crate time;
extern crate lazy_static;
extern crate log;
extern crate winapi;
extern crate winit;

mod game;
mod mainwnd;
mod rendermanager;
mod filesystem;
mod resourcemanager;
mod camera;
mod math;
mod scenemanager;
mod rhi;
mod whi;

fn main() {
    use rhi::dx12::render;
    println!("{}", render::hello());

    let mut app = game::Game::instance();
    app.create();
    app.run();
}