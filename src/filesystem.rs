use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[allow(dead_code)]
enum FileStatus
{
	Ok,           			
	IOError,          		
	EOS,              		
	IllegalCall,      		
	Closed,           		
	UnknownError
}

#[allow(dead_code)]
enum AccessMode
{
    Read,  
    Write,
    ReadWrite, 
    WriteAppend,
}

pub fn open(p: &str) -> ::std::string::String {
	let path = Path::new(p);
	let display = path.display();

	// Open the path in read-only mode, returns `io::Result<File>`
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
        Ok(_) => s,
    }
}

#[allow(dead_code)]
pub fn read_to_buffer(p: &str) -> ::std::string::String {
    let path = Path::new(p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
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
        Ok(_) => s,
    }
}

#[cfg(test)]
mod tests {
	use filesystem;
	#[test]
	#[should_panic]
    fn valid_gles_versions() {
        // only irregular version
        assert_eq!(filesystem::open("heheh"),"hh");
    }
}