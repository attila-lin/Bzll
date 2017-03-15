//const ROOT_PATH = "../";

struct PathConfig{
    ROOT_PATH: String,
}

impl PathConfig{
    pub fn new() -> Self {
        let ROOT_PATH = env!("CARGO_PKG_VERSION");
        PathConfig{
            ROOT_PATH: ROOT_PATH,
        }
    }
}
