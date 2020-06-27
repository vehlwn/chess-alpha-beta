# chess-alpha-beta

Chess solver based on minimax algorithm with alpha-beta pruning optimization. For the minimax details refer to [here](https://en.wikipedia.org/wiki/Minimax#Pseudocode).

White player is maximizer. Black is minimizer. Total score evaluated in [here](https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html#method.psq).

One can control computer's intelligence by increasing or decreasing DEPTH variable. Be aware: the algorithm working time depends on DEPTH exponentially.

# Build

```
$ cargo run --release

```
