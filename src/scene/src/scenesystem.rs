use std::sync::{Arc, Mutex};

use render::camera::Camera;

#[derive(Clone,Debug)]
pub struct SceneManager {
    pub inner: Arc<Mutex<u8>>,
    cameras: Arc<Mutex<Vec<Camera>>>,
}

impl SceneManager{
    fn new() -> Self {
        SceneManager {
            inner: Arc::new(Mutex::new(0)),
            cameras: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn init(&mut self){
        self.add_camera(Camera::new());
    }

    fn add_camera(&mut self, camera: Camera){
        self.cameras.lock().unwrap().push(camera);
        // println!("length {}", self.cameras.lock().unwrap().len());
    }

    fn get_camera(&self, index: usize) -> Camera {
        // println!("!!!!!!!{} {}", index, self.cameras.lock().unwrap().len());
        (self.cameras.lock().unwrap())[index].clone()
    }

    fn update(&self) {
        (self.cameras.lock().unwrap())[0].update()
    }

//    fn process_input(&self, event: &glutin::Event) {
//        (self.cameras.lock().unwrap())[0].process_input(event)
//    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_load_obj() {
    }
}
