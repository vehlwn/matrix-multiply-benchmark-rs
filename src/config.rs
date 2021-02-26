use clap;

pub struct Config {
    pub n: usize,
    pub r: usize,
}

pub fn parse_command_line() -> Config {
    let usize_validator = |s: String| {
        if s.parse::<usize>().is_ok() {
            return Ok(());
        }
        return Err("Invalid size".to_string());
    };
    let matches = clap::App::new("matrix-multiply-benchmark-rs")
        .about("Multiplies matricies and shows time")
        .arg(
            clap::Arg::with_name("size")
                .long("size")
                .short("n")
                .help("Width of a matrix n x n")
                .required(true)
                .takes_value(true)
                .default_value("1000")
                .validator(usize_validator.clone()),
        )
        .arg(
            clap::Arg::with_name("repeats")
                .long("repeats")
                .short("r")
                .help("Repeats operation r times")
                .required(true)
                .takes_value(true)
                .default_value("10")
                .validator(usize_validator.clone()),
        )
        .get_matches();
    let n: usize = matches.value_of("size").unwrap().parse().unwrap();
    let r: usize = matches.value_of("repeats").unwrap().parse().unwrap();
    return Config { n, r };
}
