#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

#[macro_use]
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
use resourcemanager::load_obj;

use std::thread;

fn main() {

    // let mainWnd = MainWnd::instance();
    let mut game = Game::instance();

    game.create();

    game.frame();

    resourcemanager::load_obj("./artist/Characters/Borderlands2-Zero/zero.obj");
}