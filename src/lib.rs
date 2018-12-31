pub struct Config {
    pub filename: String,
    pub sum: bool,
    pub avg: bool
}

enum Flag {
    Sum,
    Avg
}

impl Flag {
    fn parse(arg: &str) -> Option<Flag> {
        match arg.to_lowercase().as_ref() {
            "-s"     => Some(Flag::Sum),
            "--sum"  => Some(Flag::Sum),
            "-m"     => Some(Flag::Avg),
            "--mean" => Some(Flag::Avg),
            "--avg"  => Some(Flag::Avg),
            _        => None
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
        let mut config = Config { filename: String::from(""), sum: false, avg: false };
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
    pub count: i64,
    pub sum: f64
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { count: 0, sum: 0.0 }
    }

    pub fn add(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
    }

    pub fn avg(&self) -> f64 {
        if self.count <= 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}
