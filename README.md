# Game of life kata

A Rust implementation of the Game of Life. This code was written for the Rust
Meetup in Milan of 19/02/2020, and then readapted to compile and run.

## Rules

1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
2. Any live cell with more than three live neighbours dies, as if by overcrowding.
3. Any live cell with two or three live neighbours lives on to the next generation.
4. Any dead cell with exactly three live neighbours becomes a live cell.

## Fun facts

* We use the GOL Builder from [Michele D'Amico](https://github.com/la10736/gol_builder).
* I convinced my teammates to implement the rules using a 2D convolution.
