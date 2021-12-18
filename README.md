# Advent of Code 2021
My [Advent of Code 2021](https://adventofcode.com) solutions in the Rust programming language.
I try to develop an elegant, compact and fast solution using Idiomatic Rust.
## Usage
```sh
# Run all the days
cargo run --release

#Run a specific day
cargo run --release --bin day01
```
## Benchmark
### Run

```sh
cargo bench
```



### Timing

|                       | Problem                                            | Part 1   | Part 2   |   
|-----------------------|----------------------------------------------------|----------|----------|
| [Day 1](src/day01.rs) | [Problem 1](https://adventofcode.com/2021/day/1)   | 0.048 ms | 0.051 ms |   
| [Day 2](src/day02.rs) | [Problem 2](https://adventofcode.com/2021/day/2)   | 0.040 ms | 0.04l ms |   

> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 13' 2015
