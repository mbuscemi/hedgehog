use std::io::prelude::*;
use std::fs::File;

pub fn read(path: String) -> String {
     let mut file = File::open(path).expect("unable to open file");
     let mut buffer = String::new();
     file.read_to_string(&mut buffer).expect("unable to read the file");
     serde_json::to_string(&buffer).expect("unable to JSON encode file contents").to_string()
}
