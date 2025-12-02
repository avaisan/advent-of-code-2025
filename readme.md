# Advent of code 2025

Learning Rust while doing these exercises, so repo will evolve and contain Rust book examples also. Trying to document basics here while at it.

## Creating a new project

Using cargo:

```
cargo new hello_world
```

## Compiling and running code:

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

## Checking build

```
cargo check
    Checking advent-of-code-2025 v1.0.0 (/xxxxx/xxxxxx/advent-of-code-2025)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
```