### AoC Rope Snake

A snake clone based on the rope physics from the [day 9](https://adventofcode.com/2022/day/9) puzzle from [Advent of code 2022](https://adventofcode.com/2022).
The code is based of the [snake example](https://github.com/not-fl3/macroquad/blob/master/examples/snake.rs) from [macroquad](https://github.com/not-fl3/macroquad).

#### Gameplay instructions

Move with `WASD` or using the arrow keys.
Pick up the green orbs to get longer.

You get a point for every square the very and of your tails visits. You also get another 50 points if you eat a orb.

If you would like to pause the game at any time, press the space bar.

#### Build instructions

##### Desktop
```
cargo run --release
```

##### Web
See [this](https://github.com/not-fl3/macroquad#wasm)

