pub struct Config {
    pub filename: String,
    pub sum: bool,
    pub mean: bool,
    pub variance: bool,
    pub stddev: bool,
    pub min: bool,
    pub max: bool,
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
            "--min"  => self.min = true,
            "--max"  => self.max = true,
            "--var"  => self.variance = true,
            "--stddev"  => self.stddev = true,
            _        => ()
        }
    }

    pub fn new(args: std::env::Args) -> Result<Config, String> {
        let mut filename_arg = None;
        let mut config = Config { filename: String::from(""), sum: false, mean: false, min: false, max: false, variance: false, stddev: false};

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
    pub sum: f64,
    pub mean: f64,
    M2: f64, // M2 aggregates the squared distance from the mean
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub variance: Option<f64>,
    pub stddev: Option<f64>,
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { count: 0, sum: 0.0, mean: 0.0, M2: 0.0, min: None, max: None, variance: None, stddev: None }
    }

    pub fn add(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.min = match self.min {
            Some(x) if x < value => Some(x),
            _                    => Some(value)
        };
        self.max = match self.min {
            Some(x) if x > value => Some(x),
            _                    => Some(value)
        };

        // compute variation with Welford's Online algorithm
        // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_Online_algorithm
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = value - self.mean;
        self.M2 += delta * delta2;
    }

    pub fn finalize(&mut self) {
        let variance = self.M2 / ((self.count - 1) as f64);
        self.variance = Some(variance);
        self.stddev = Some(variance.sqrt());
    }
}
