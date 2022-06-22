mod alpha_beta;
mod board_pretty_print;
mod board_value;
mod config;
mod game_modes;
mod shuffled_move_list;

use game_modes::black_user_with_white_computer;
use game_modes::computer_with_computer;
use game_modes::white_user_with_black_computer;

fn main() {
    let config = config::parse_command_line();
    println!("depth = {}", config.depth);
    match config.mode.as_ref() {
        "cc" => computer_with_computer(config),
        "wubc" => white_user_with_black_computer(config),
        "buwc" => black_user_with_white_computer(config),
        x => panic!("Unknown mode: '{}'", x),
    };
}
