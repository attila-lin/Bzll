#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate rand;
extern crate fnv;

pub mod types;
pub mod render_system;
pub mod camera;
//pub mod material;
//pub mod sphere;
//pub mod texture;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
