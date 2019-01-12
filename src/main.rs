extern crate st;
use st::Config;
use st::Accumulator;
use std::env;

fn read_file(config: &Config) {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = File::open(&config.filename).unwrap();
    let mut acc = Accumulator::new();
    for line in BufReader::new(file).lines() {
        let value: f64 = line.unwrap().parse().unwrap();
        acc.add(value);
    }
    acc.finalize();

    if config.sum {
        println!("sum: {}", acc.sum);
    }

    if config.mean {
        println!("avg: {}", acc.mean);
    }

    match acc.variance {
        Some(x) if config.variance => println!("variance: {}", x),
        _ => ()
    }

    match acc.stddev {
        Some(x) if config.stddev => println!("stddev: {}", x),
        _ => ()
    }

    match acc.min {
        Some(x) if config.min => println!("min: {}", x),
        _ => ()
    }

    match acc.max {
        Some(x) if config.max => println!("min: {}", x),
        _ => ()
    }
}

fn main() {
    let args = env::args();
    let config = st::Config::new(args).unwrap();
    println!("Config: {}, sum = {}", &config.filename, config.sum);
    read_file(&config);;
}
