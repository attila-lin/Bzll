use whi::window::{Window, InitError};
use winit;

pub fn init() -> winit::WindowBuilder{

    let wb = winit::WindowBuilder::new()
            .with_title("Game".to_string())
            .with_dimensions(1024, 768);

    return wb;
//    let inner = match wb.build() {
//        Ok(w) => w,
//        Err(_) => return Err(InitError::Window),
//    };
//
//    let (width, height) = inner.get_inner_size_pixels().unwrap();
//
//    // let swap_chain =
//
//    let window = Window{
//        inner: inner,
//        // swap_chain: swap_chain,
//    };
//
//    return Ok((window));
}
