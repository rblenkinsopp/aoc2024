use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn get_input() -> BufReader<File> { 
    BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
}

