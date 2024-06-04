mod alpha_beta;
mod board_pretty_print;
mod board_value;
mod config;
mod game_modes;

use clap::Parser;
use config::{Config, GameMode};
use game_modes::{
    black_user_with_white_computer, computer_with_computer,
    white_user_with_black_computer,
};

fn main() {
    let config = Config::parse();
    match config.mode {
        GameMode::CC => computer_with_computer(config),
        GameMode::WUBC => white_user_with_black_computer(config),
        GameMode::BUWC => black_user_with_white_computer(config),
    };
}
