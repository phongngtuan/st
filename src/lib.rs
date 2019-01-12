pub struct Config {
    pub filename: String,
    pub sum: bool,
    pub mean: bool
}

impl Config {
    fn parse_short_flags(&mut self, flags: &str) {
        for c in flags.chars().skip(1) {
            println!("{}", c);
            match c {
                's' => self.sum = true,
                'm' => self.mean = true,
                _   => ()
            }
        }
    }

    fn parse_long_flags(&mut self, flags: &str) {
        match flags.as_ref() {
            "--sum"  => self.sum = true,
            "--mean" => self.mean = true,
            "--avg"  => self.mean = true,
            _        => ()
        }
    }

    pub fn new(args: std::env::Args) -> Result<Config, String> {
        let mut filename_arg = None;
        let mut config = Config { filename: String::from(""), sum: false, mean: false };

        for arg in args {
            if !arg.starts_with("-") {
                filename_arg = Some(arg);
            } else if arg.starts_with("--") {
                config.parse_long_flags(&arg);
            } else {
                config.parse_short_flags(&arg);
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
