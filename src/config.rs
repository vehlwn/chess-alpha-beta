pub struct Config {
    pub depth: i32,
    pub evaluate_user: bool,
    pub mode: String,
}

pub fn parse_command_line() -> Config {
    let matches = clap::App::new("chess-alpha-beta")
        .arg(
            clap::Arg::with_name("depth")
                .long("depth")
                .short("d")
                .help("depth of search tree")
                .default_value("6")
                .validator(|s| {
                    if let Ok(n) = s.parse::<i32>() {
                        if n > 0 {
                            return Ok(());
                        }
                    }
                    return Err("depth must be positive integer".to_owned());
                }),
        )
        .arg(
            clap::Arg::with_name("mode")
                .long("mode")
                .short("m")
                .help("game mode: Computer-Computer, White User-Black Computer, Black User-White Computer")
                .possible_values(&["cc", "wubc", "buwc"])
                .default_value("wubc"),
        )
        .arg(
            clap::Arg::with_name("evaluate_user")
                .long("evaluate-user")
                .short("e")
                .help("show user's potentially best move when playing with computer")
                .takes_value(true)
                .possible_values(&["0", "1"])
                .default_value("0"),
        )
       .get_matches();
    let depth: i32 = matches.value_of("depth").unwrap().parse().unwrap();
    let evaluate_user: bool = matches
        .value_of("evaluate_user")
        .unwrap()
        .parse::<i32>()
        .unwrap()
        != 0;
    let mode = matches.value_of("mode").unwrap();
    return Config {
        depth,
        evaluate_user,
        mode: mode.to_string(),
    };
}
