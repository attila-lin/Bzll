#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

mod game;
mod mainwnd;
mod rendermanager;
mod filesystem;
mod resourcemanager;

use game::Game;
// use mainwnd::MainWnd;
use resourcemanager::load_obj;

use std::thread;

fn main() {

    // let ioThread = thread::spawn(move || {
    //     let s = FileManager::instance();
    //     let mut data = s.inner.lock().unwrap();
    //     print!("hehe");
    // });

    // let mainWnd = MainWnd::instance();
    let mut game = Game::instance();

    game.create();

    game.frame();

    resourcemanager::load_obj("./artist/Characters/Borderlands2-Zero/zero.obj");
}