# Advent of Code, 2023

My implementation of the puzzles from [Advent of Code 2023](https://adventofcode.com/2023) in Rust.

## Usage

If, for some reason, you wanted to run my implementations, that's possible!
All the days are "pre-implemented", which just means they have stub function set up
which just return a custom "Unimplemented" error type.

The first command-line option is a string of the form `day` or `day.part` where:
- 1 <= day <= 25, and 
- 1 <= part <= 2

If the first option is omitted, all implemented days and parts will be ran.

### Examples

Run everything that's been implemented so far:
```
cargo run
```

Run day 1, part 1:
```
cargo run 1.1
```

Run day 12, part 2:
```
cargo run 12.2
```

Run both parts for day 7:
```
cargo run 7
```
