use chess_alpha_beta::config::{Config, GameMode};
use chess_alpha_beta::game_modes::{
    black_user_with_white_computer, computer_with_computer,
    white_user_with_black_computer,
};
use clap::Parser;

fn main() {
    let config = Config::parse();
    match config.mode {
        GameMode::CC => computer_with_computer(config),
        GameMode::WUBC => white_user_with_black_computer(config),
        GameMode::BUWC => black_user_with_white_computer(config),
    };
}
