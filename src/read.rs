
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

/// simply reads a JSON file
pub fn read_json_file(x: &str) -> String {
    // Create a path to the test file
    let path = Path::new(x);
    let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.to_string()),
        Ok(file) => file,
    };


    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.to_string()),
        Ok(_) => return s
    }
}