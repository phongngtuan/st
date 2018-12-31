extern crate st;
use st::*;
use std::env;

fn read_file(filename: &str) {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = File::open(filename).unwrap();
    for line in BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    let args = env::args();
    let config = st::Config::new(args).unwrap();
    println!("Config: {}, sum = {}", config.filename, config.sum);
    read_file(&config.filename);
}
