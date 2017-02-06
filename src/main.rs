#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

mod game;
mod mainwnd;
mod rendermanager;
mod filesystem;
mod resourcemanager;

// use game::Game;
// use mainwnd::MainWnd;
use rendermanager::RenderManager;
use resourcemanager::load_obj;

use std::thread;

fn main() {

    // let ioThread = thread::spawn(move || {
    //     let s = FileManager::instance();
    //     let mut data = s.inner.lock().unwrap();
    //     print!("hehe");
    // });

    // let mainWnd = MainWnd::instance();
    // let game = Game::instance();

    resourcemanager::load_obj("./artist/Characters/Borderlands2-Zero/zero.obj");
    
    let renderThread = thread::spawn(move || {
        let renderManager = RenderManager::instance();
        renderManager.startUp()
    });

    renderThread.join().unwrap();
}