pub struct Config {
    pub filename: String,
    pub sum: bool
}

enum Flag {
    Sum
}

impl Flag {
    fn parse(arg: &str) -> Option<Flag> {
        match arg.to_lowercase().as_ref() {
            "-s"    => Some(Flag::Sum),
            "--sum" => Some(Flag::Sum),
            _       => None
        }
    }
}

impl Config {
    fn set_config_flag(arg: &str, config: &mut Config) {
        match Flag::parse(arg) {
            Some(Flag::Sum) => config.sum = true,
            _ => ()
        }
    }

    pub fn new(args: std::env::Args) -> Result<Config, String> {
        let mut filename_arg = None;
        let mut config = Config { filename: String::from(""), sum: false };
        for arg in args {
            if !arg.starts_with("-") {
                filename_arg = Some(arg);
            } else {
                Config::set_config_flag(&arg, &mut config);
            }
        }
        match filename_arg {
            None => {
                Err(String::from("no filename provided"))
            }
            Some(s) => {
                config.filename = s;
                Ok(config)
            }
        }
    }
}

pub struct Accumulator {
    pub sum: f64
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { sum: 0.0 }
    }

    pub fn add(&mut self, value: f64) {
        self.sum += value;
    }
}
