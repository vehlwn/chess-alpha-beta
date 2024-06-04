#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum GameMode {
    /// Computer-Computer
    CC,
    /// White User-Black Computer
    WUBC,
    /// Black User-White Computer
    BUWC,
}

/// Chess solving program based on minimax algorithm with alpha-beta pruning optimization
#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Depth of a search tree
    #[arg(short, long, default_value = "6")]
    pub depth: std::num::NonZeroU32,

    /// Show user's potentially best move when playing with computer
    #[arg(short, long)]
    pub evaluate_user: bool,

    /// Game mode
    #[arg(short, long)]
    pub mode: GameMode,
}
