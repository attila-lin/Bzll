#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

mod game;
mod mainwnd;
mod rendermanager;
mod filesystem;

use game::Game;
use mainwnd::MainWnd;
use rendermanager::RenderManager;

use std::time::Duration;
use std::{mem, thread};

// static STATIC: Type = init;


fn main() {

    // let ioThread = thread::spawn(move || {
    //     let s = FileManager::instance();
    //     let mut data = s.inner.lock().unwrap();
    //     print!("hehe");
    // });

    let mainWnd = MainWnd::instance();
    let game = Game::instance();
    
    let renderThread = thread::spawn(move || {
        let renderManager = RenderManager::instance();
        // let mut data = s.inner.lock().unwrap();
        // print!("hehe");
        renderManager.startUp()
    });

    renderThread.join().unwrap();
}