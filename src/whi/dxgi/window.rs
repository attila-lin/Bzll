use glutin;

pub fn init<'a>() -> glutin::WindowBuilder<'a>{

    let wb = glutin::WindowBuilder::new()
            .with_title("Game".to_string())
            .with_dimensions(1024, 768)
            .with_vsync();

    return wb;
}
