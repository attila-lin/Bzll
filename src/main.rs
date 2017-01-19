#[macro_use]
extern crate glium;
extern crate image;

mod rendermanager;

use rendermanager::RenderManager;

use std::time::Duration;
use std::{mem, thread};


fn main() {

    // let ioThread = thread::spawn(move || {
    //     let s = FileManager::instance();
    //     let mut data = s.inner.lock().unwrap();
    //     print!("hehe");
    // });
    
    let renderThread = thread::spawn(move || {
        let rendermanager = RenderManager::instance();
        // let mut data = s.inner.lock().unwrap();
        // print!("hehe");
        rendermanager.startUp()
    });

    renderThread.join().unwrap();
}