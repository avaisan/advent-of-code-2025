# Advent of code 2025

Learning Rust while doing these exercises, so repo will evolve and contain Rust book examples also. Trying to document basics here while at it.

## Rust book examples

Using cargo to create a new project:

```
cargo new hello_world
```

### Compiling and running code:

Compile:
```
cargo build --bin hello_world
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s

./target/debug/hello_world
# Hello world!
```

Optionally build release with optimizations:
```
cargo build --release
```

Compile and run:
```
cargo run --bin hello_world
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
# Running `target/debug/hello_world`
# Hello world!
```

### Checking build

```
cargo check
    Checking advent-of-code-2025 v1.0.0 (/xxxxx/xxxxxx/advent-of-code-2025)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
```


## Advent of code

### Day 1
- Test cases / test descriptions were written first to validate dial position after rotations and counting how many times it hit 0.
- Then implementation was done to match them. Learning curve was fairly high here on structuring files, getting compilation working and learning modulo using Rust.

```
cargo test --bin day1
cargo run --bin day1
```

### Day 2
- Part 1: Problem framing took longest. Syntax to solve the code was nearly identical to day 1. Just had to read on Rust data types more. Started again by writing simple test cases with expectations first.

```
cargo test --bin day2
cargo run --bin day2
```