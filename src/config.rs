#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Default)]
pub struct Conf {
    pub window_title: String,
    pub window_icon: String,
    pub window_height: u32,
    pub window_width: u32,
}

// TODO graph config
//