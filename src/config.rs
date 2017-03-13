use std::io;
use std::io::prelude::*;
use std::fs::File;

use toml:: { Parser, Decoder, Value };
use rustc_serialize:: { Decodable };

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Default)]
pub struct Config {
    app: AppConfig,
    window: WinConfig,
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Default)]
pub struct AppConfig {
    //    pub window_type: WindowType,
    pub name: Option<String>,
}


#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Default)]
pub struct WinConfig {
//    pub window_type: WindowType,
    pub window_title: String,
    pub window_height: u32,
    pub window_width: u32,
}

// TODO graph config
//

impl Config {
    pub fn get_config() -> Self
    {
        let mut file = File::open("src/config.toml").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let v = Parser::new(&buffer).parse().unwrap();
        let mut d = Decoder::new(Value::Table(v));
        let config = Config::decode(&mut d).unwrap();

        assert_eq!(config.app.name, Some("game".to_string()));

        config
    }
}