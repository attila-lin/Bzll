// Resource GUIDs
// Resource File Formats
// Resource File and Directory Organization
// Responsibilities of the Runtime Resource Manage
// The Resource Registry

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::any::{Any, TypeId};

use fnv::FnvHashMap as HashMap;

pub struct Assets {
    loaders: HashMap<TypeId, Box<Any>>,
//    asset_ids: HashMap<String, AssetId>,
//    assets: World,
}

impl Assets {
    fn new() -> Assets {
        Assets {
            loaders: HashMap::default(),
        }
    }
}

pub struct ResourcSystem{
    assets: Assets,
}

impl ResourcSystem {
    pub fn new() -> Self{
        ResourcSystem{
            assets: Assets::new(),
        }
    }
}

fn load_file(name: &str) -> Vec<u8> {
    let mut file = File::open(name).unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();

    buffer
}

// http://www.opengl-tutorial.org/beginners-tutorials/tutorial-7-model-loading/
pub fn load_obj(p: &str){
	let path = Path::new(p);
	let display = path.display();

	let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => &s,
    };

    // let mut lines = text.lines();
    // loop {
    // 	let mut line = lines.next();
    // }
    let split = s.split("\n");
    let vec: Vec<&str> = split.collect();
    for si in &vec {
	    // println!("{}", si);
	    let mut v = si.split_whitespace();
	    match v.next() {
	    	Some("v") => println!("v"),
	    	Some("vt") => println!("vt"),
	    	Some("vn") => println!("vn"),
	    	// Some("v") => println!("v"),
	    	None => break,
            Some(&_) => break,
	    }
	    // n = v.next();
	}
}

//pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBufferAny {
//    #[derive(Copy, Clone)]
//    struct Vertex {
//        position: [f32; 3],
//        normal: [f32; 3],
//        texture: [f32; 2],
//    }
//
//    implement_vertex!(Vertex, position, normal, texture);
//
//    let mut data = ::std::io::BufReader::new(data);
//    let data = obj::Obj::load(&mut data);
//
//    let mut vertex_data = Vec::new();
//
//    for object in data.object_iter() {
//        for shape in object.group_iter().flat_map(|g| g.indices().iter()) {
//            match shape {
//                &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
//                    for v in [v1, v2, v3].iter() {
//                        let position = data.position()[v.0];
//                        let texture = v.1.map(|index| data.texture()[index]);
//                        let normal = v.2.map(|index| data.normal()[index]);
//
//                        let texture = texture.unwrap_or([0.0, 0.0]);
//                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);
//
//                        vertex_data.push(Vertex {
//                            position: position,
//                            normal: normal,
//                            texture: texture,
//                        })
//                    }
//                },
//                _ => unimplemented!()
//            }
//        }
//    }
//
//    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into_vertex_buffer_any()
//}

#[cfg(test)]
mod tests {

    use super::*;
    fn test_load_obj() {
        // only irregular version
        load_obj("./artist/Characters/Borderlands2-Zero/zero.obj");
    }
}
