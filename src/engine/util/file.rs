use std::fs::File;
use std::io::{Write, BufWriter, Read, BufReader};
use std::io::prelude::*;
use engine::util;

pub fn read_file(file_path : &'static str) -> String {
    // create empty string
    let mut data = String::new();
    // read file in from path and parse buffer
    let file_buffer = File::open(file_path).expect("Unable to open file");
    let mut file_data = BufReader::new(file_buffer);
    // move file buffer as `String` into string_buffer
    file_data.read_to_string(&mut data).expect("Unable to read string");
    // return data
    data
}

pub fn read_file_buffer(file_path : &'static str) -> BufReader<File>{
    // create empty string
    let mut data = String::new();
    // read file in from path and parse buffer
    let file_buffer = File::open(file_path).expect("Unable to open file");
    let mut file_data = BufReader::new(file_buffer);
    // return buffer
    file_data
}

pub fn write_file(file_path: &'static str, data: String) {
    // create file in directory path and parse buffer
    let file_buffer = File::create(file_path).expect("Unable to create file");
    let mut file_data = BufWriter::new(file_buffer);
    // write data into file
    file_data.write_all(data.as_bytes()).expect("Unable to write data");
}