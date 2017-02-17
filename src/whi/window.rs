use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use winit;

pub trait IWindow{
    fn init(&mut self);
}

pub struct Window {
    pub inner: winit::Window,
}

#[derive(Copy, Clone, Debug)]
pub enum InitError {
    /// Unable to create a window.
    Window,
    /// Unable to map format to DXGI.
    // Format(format::Format),
    /// Unable to find a supported driver type.
    DriverType,
}

impl Window {
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_load_obj() {
    }
}
