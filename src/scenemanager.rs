use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;

use camera::Camera;

#[derive(Clone,Debug)]
pub struct SceneManager {
    pub inner: Arc<Mutex<u8>>,
    cameras: Arc<Mutex<Vec<Camera>>>,
}

pub trait Interface {
    fn add_camera(&mut self, camera: Camera);
    fn get_camera(&self, index: usize) -> Camera;
}

impl SceneManager {
    fn new() -> Self {
        SceneManager {
            inner: Arc::new(Mutex::new(0)),
            cameras: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn instance() -> Box<Interface> {
        // Initialize it to a null value
        static mut SINGLETON: *const SceneManager = 0 as *const SceneManager;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                println!("SceneManager");

                // Put it in the heap so it can outlive this call
                SINGLETON = mem::transmute(Box::new(SceneManager::new()));
            });

            // Now we give out a copy of the data that is safe to use concurrently.
            Box::new((*SINGLETON).clone())
        }
    }
}

impl Interface for SceneManager {
    fn add_camera(&mut self, camera: Camera){
        self.cameras.lock().unwrap().push(camera);
        // println!("length {}", self.cameras.lock().unwrap().len());
    }

    fn get_camera(&self, index: usize) -> Camera {
        // println!("!!!!!!!{} {}", index, self.cameras.lock().unwrap().len());
        (self.cameras.lock().unwrap())[index].clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_load_obj() {
    }
}
