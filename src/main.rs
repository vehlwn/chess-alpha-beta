mod alpha_beta;
mod board_pretty_print;
mod board_value;
mod game_modes;
mod shuffled_move_list;

use clap;
use game_modes::black_user_with_white_computer;
use game_modes::computer_with_computer;
use game_modes::white_user_with_black_computer;

fn main() {
    const VERBOSE: bool = false;

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
            clap::Arg::with_name("verbose")
                .short("v")
                .help("verbosity level")
                .possible_values(&["0", "1"])
                .default_value("0"),
                )
        .get_matches();
    let depth: i32 = matches.value_of("depth").unwrap().parse().unwrap();
    println!("depth = {}", depth);

    let verbose: bool =
        matches.value_of("verbose").unwrap().parse::<i32>().unwrap() == 1;

    let mode = matches.value_of("mode").unwrap();
    match mode {
        "cc" => computer_with_computer(depth, verbose),
        "wubc" => white_user_with_black_computer(depth, verbose),
        "buwc" => black_user_with_white_computer(depth, verbose),
        x => panic!("Unknown mode: '{}'", x),
    };
}
