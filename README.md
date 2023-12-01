My rust template repo for the Advent of Code. Click the `Use this template` button to get your own up and running? or don't - up to you.

## Usage

### Creating a template for a new day

To create a new template for a given day, simply run:

```
cargo run n <day>
```

where <day> is the day you want to create. Example, where 3 is for day 3 of of the advent of code 2023. This will create:

- your `days/three/one.rs` file: which is where you will write your puzzle 1 implementation for the day.
- your `days/three/two.rs` file: which is where you will write your puzzle 2 implementation for the day.
- your `days/three/input.txt` file, which is where you will put your input for the day.
- your `days/three/mod.rs` file, you can ignore this if you want.

### Run the day

To run the program for a given day, simply run:

```
cargo run r <day> <puzzle>
```

where <day> is a number for the day you want to run, and puzzle is `1` or `2`.

Example, where 3 is for day 3 of of the advent of code 2023 (3rd December) and you want to run the second puzzle, you would run:

```
cargo run r 3 2
```
