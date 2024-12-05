use std::{env, fs};
use std::fs::File;
use std::io::BufReader;

pub fn get_input_reader() -> BufReader<File> {
    BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
}

pub fn get_input_as_string() -> String {
    fs::read_to_string(env::args().nth(1).unwrap()).unwrap()
}

