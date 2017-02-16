#[macro_use]
extern crate glium;
extern crate image;
extern crate time;
extern crate winit;
extern crate lazy_static;


mod game;
mod mainwnd;
mod rendermanager;
mod filesystem;
mod resourcemanager;
mod camera;
mod vector;
mod scenemanager;



use game::Game;
// use mainwnd::MainWnd;
// use resourcemanager::load_obj;

fn main() {

    let mut game = Game::instance();
    game.create();
    game.run();

    // resourcemanager::load_obj("./artist/Characters/Borderlands2-Zero/zero.obj");
}