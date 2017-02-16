#[macro_use]
extern crate glium;
extern crate image;
extern crate time;
extern crate lazy_static;
extern crate log;
extern crate dxguid;
extern crate winapi;
extern crate winit;

mod game;
mod mainwnd;
mod rendermanager;
mod filesystem;
mod resourcemanager;
mod camera;
mod vector;
mod scenemanager;
mod rhi;
// use mainwnd::MainWnd;
// use resourcemanager::load_obj;

fn main() {
    use rhi::dx12::render;
    println!("{}", render::hello());

    let mut app = game::Game::instance();
    app.create();
    app.run();

    // resourcemanager::load_obj("./artist/Characters/Borderlands2-Zero/zero.obj");
}