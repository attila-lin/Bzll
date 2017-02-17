use whi::window::{Window, InitError};
use winit;

pub fn init() -> Result<(Window), InitError>{

    let wb = winit::WindowBuilder::new().with_title("Game".to_string());
    let inner = match wb.build() {
        Ok(w) => w,
        Err(_) => return Err(InitError::Window),
    };
    let window = Window{
        inner: inner,
    };

    return Ok((window));
}
