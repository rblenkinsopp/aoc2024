use std::fs::File;
use std::io::BufReader;
use std::{env, fs};

#[inline]
pub fn get_input_reader() -> BufReader<File> {
    BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
}

#[inline]
pub fn get_input_as_string() -> String {
    fs::read_to_string(env::args().nth(1).unwrap()).unwrap()
}
