# chess-alpha-beta

Chess solver based on minimax algorithm with alpha-beta pruning optimization. For the minimax details refer to [here](https://en.wikipedia.org/wiki/Minimax#Pseudocode).

White player is maximizer. Black is minimizer. Total score evaluated in [here](https://docs.rs/pleco/0.5.0/pleco/board/struct.Board.html#method.psq).

One can control computer's intelligence by increasing or decreasing DEPTH variable. Be aware: the algorithm working time depends on DEPTH exponentially.

# Build

```
$ cargo run --release
===== 0-th move:
8| r n b q k b n r
7| p p p p p p p p
6| . . . . . . . .
5| . . . . . . . .
4| . . . . . . . .
3| . . . . . . . .
2| P P P P P P P P
1| R N B Q K B N R
------------------
 | a b c d e f g h
legal_moves = ["a2a3", "a2a4", "b1a3", "b1c3", "b2b3", "b2b4",
"c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3", "f2f4",
 "g1f3", "g1h3", "g2g3", "g2g4", "h2h3", "h2h4"], len = 20
Type white move: g1f3
Black value = 199
black move = g8f6
===== 2-th move:
8| r n b q k b . r
7| p p p p p p p p
6| . . . . . n . .
5| . . . . . . . .
4| . . . . . . . .
3| . . . . . N . .
2| P P P P P P P P
1| R N B Q K B . R
------------------
 | a b c d e f g h
legal_moves = ["a2a3", "a2a4", "b1a3", "b1c3", "b2b3", "b2b4",
"c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f3d4", "f3e5",
 "f3g1", "f3g5", "f3h4", "g2g3", "g2g4", "h1g1", "h2h3", "h2h4"
], len = 22
Type white move: c2c4
Black value = 167
black move = d7d5
...
(two hours later)
...
==== 52-th move:
8| . . . r k b . r
7| p p . . p . p p
6| . . p . n . . R
5| . . . . . . . .
4| . P n . . . . .
3| P . . . . . . .
2| . . p . . P . .
1| R . . . K . q .
------------------
 | a b c d e f g h
legal_moves = ["e1e2"], len = 1
Type white move: e1e2
Black value = -1006731
black move = g1d1
===== 54-th move:
8| . . . r k b . r
7| p p . . p . p p
6| . . p . n . . R
5| . . . . . . . .
4| . P n . . . . .
3| P . . . . . . .
2| . . p . K P . .
1| R . . q . . . .
------------------
 | a b c d e f g h
legal_moves = ["a1d1"], len = 1
Type white move: a1d1
Black value = -1006731
black move = c2d1q
Chechmate! Black won!
```
