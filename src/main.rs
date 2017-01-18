mod rendermanager;

use rendermanager::{RenderManager, singleton};

use std::time::Duration;
use std::{mem, thread};


fn main() {
    // Let's use the singleton in a few threads
    let threads: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 10));
            let s = singleton();
            let mut data = s.inner.lock().unwrap();
            *data = i as u8;
        })
    }).collect();

    // And let's check the singleton every so often
    for _ in 0u8..20 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("It is: {}", *data);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}